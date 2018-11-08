#[macro_export]
macro_rules! compose_sprite {
    ($class_name: ident, $sprite_name: ident, $builder: expr) => {
        pub struct $class_name<'a> {
            base: $sprite_name<'a>,
        }

        impl<'a> $class_name<'a> {
            pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
                Self {
                    base: $builder(main_renderer),
                }
            }

            pub fn resize(&mut self, size: &u32) {
                self.base.resize(size);
            }
        }

        impl<'a> RenderPosition for $class_name<'a> {
            fn render_on(&mut self, x: &usize, y: &usize) {
                self.base.render_on(x, y);
            }

            fn move_by(&mut self, x: i32, y: i32) {
                let dest = &mut self.base.renderable.dest;
                let c = { dest.clone() };
                let (dx, dy) = { (c.x(), c.y()) };
                dest.set_x(dx + x);
                dest.set_y(dy + y);
            }

            fn move_to(&mut self, x: i32, y: i32) {
                let dest = &mut self.base.renderable.dest;
                dest.set_x(x);
                dest.set_y(y);
            }
        }

        impl<'a> Sprite<'a> for $class_name<'a> {
            fn update(&mut self, ticks: i32) {
                self.base.update(ticks);
            }

            fn render(
                &mut self,
                canvas: &mut WindowCanvas,
                main_renderer: &mut MainRenderer<'a, 'a>,
            ) {
                self.base.render(canvas, main_renderer);
            }
        }
    };
}
