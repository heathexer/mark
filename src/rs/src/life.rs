use ndarray::{s, Array, Array2, Zip};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use queues::{Buffer, IsQueue};
use rpi_led_matrix::{LedCanvas, LedColor};

pub struct LifeWidget {
    size: (usize, usize),
    position: (usize, usize),
    alive_color: LedColor,
    dead_color: LedColor,
    board: Array2<u8>,
    oldboards: Buffer<Array2<u8>>,
}

impl LifeWidget {
    pub fn new(position: (usize, usize), size: (usize, usize)) -> Self {
        let mut oldboards_init = Buffer::<Array2<u8>>::new(2);
        oldboards_init.add(Array2::zeros(size));
        oldboards_init.add(Array2::zeros(size));
        LifeWidget {
            size: size,
            position: position,
            alive_color: LedColor {
                red: 50,
                green: 0,
                blue: 0,
            },
            dead_color: LedColor {
                red: 0,
                green: 0,
                blue: 0,
            },
            board: Array::random(size, Uniform::new(0_u8, 2_u8)),
            oldboards: oldboards_init,
        }
    }

    fn count_board(&self) -> Array2<u8> {
        let mut res = Array2::<u8>::zeros(self.size);

        for arr in [
            shift_copy(&self.board, 1, 0),
            shift_copy(&self.board, 1, -1),
            shift_copy(&self.board, 0, -1),
            shift_copy(&self.board, -1, -1),
            shift_copy(&self.board, -1, 0),
            shift_copy(&self.board, -1, 1),
            shift_copy(&self.board, 0, 1),
            shift_copy(&self.board, 1, 1),
        ] {
            res += &arr;
        }

        res
    }

    fn update(&mut self) {
        let mut new_board = Array2::<u8>::zeros(self.size);

        let counts = self.count_board();

        azip!((new in &mut new_board, count in &counts, status in &self.board) {
            *new =
            if *status == 1 {
                if *count == 2 || *count == 3 {1} else {0}
            } else {
                if *count == 3 {1} else {0}
            }
        });

        // Reset on steady state

        if new_board == self.oldboards.remove().unwrap() {
            self.board = Array::random(self.size, Uniform::new(0_u8, 2_u8));
        } else {
            self.board = new_board;
        }
        self.oldboards.add(self.board.clone());
    }

    pub fn render(&mut self, canvas: &mut LedCanvas) {
        self.update();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                canvas.set(
                    self.position.0 as i32 + x as i32,
                    self.position.1 as i32 + y as i32,
                    if self.board[[x, y]] == 1 {
                        &self.alive_color
                    } else {
                        &self.dead_color
                    },
                )
            }
        }
    }
}

fn shift_copy(bord: &Array2<u8>, x: isize, y: isize) -> Array2<u8> {
    let mut res = Array2::maybe_uninit(bord.dim());
    let mut tmp = Array2::maybe_uninit(bord.dim());

    if x == 0 {
        assign_to(bord, &mut tmp)
    } else {
        assign_to(bord.slice(s![-x.., ..]), tmp.slice_mut(s![..x, ..]));
        assign_to(bord.slice(s![..-x, ..]), tmp.slice_mut(s![x.., ..]));
    }

    if y == 0 {
        assign_to(&tmp, &mut res);
    } else {
        assign_to(tmp.slice(s![.., -y..]), res.slice_mut(s![.., ..y]));
        assign_to(tmp.slice(s![.., ..-y]), res.slice_mut(s![.., y..]));
    }

    unsafe { res.assume_init() }
}

// Stolen from example
use ndarray::{AssignElem, IntoNdProducer};

fn assign_to<'a, P1, P2, A>(from: P1, to: P2)
where
    P1: IntoNdProducer<Item = &'a A>,
    P2: IntoNdProducer<Dim = P1::Dim>,
    P2::Item: AssignElem<A>,
    A: Clone + 'a,
{
    Zip::from(from).apply_assign_into(to, A::clone);
}
