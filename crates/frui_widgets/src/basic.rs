use frui::prelude::*;

#[derive(RenderWidget, Builder)]
pub struct ColoredBox<T: Widget> {
    pub child: T,
    pub color: Color,
}

impl<T: Widget> RenderWidget for ColoredBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.smallest()
        }
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let rect = Rect::from_origin_size(*offset, ctx.size());
        let brush = &canvas.solid_brush(self.color.clone());
        canvas.fill(rect, brush);
        ctx.child(0).paint(canvas, offset)
    }
}
