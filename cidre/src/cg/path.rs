use crate::{arc, cf, cg, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ElementType {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubpath,
}

define_cf_type!(Path(cf::Type));
impl Path {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGPathGetTypeID() }
    }

    #[inline]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { CGPathCreateCopy(self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopy(self) }
    }

    #[inline]
    pub fn copy_transforming_path(&self, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateCopyByTransformingPath(self, transform) }
    }

    #[inline]
    pub fn copy_mut_transforming_path(
        &self,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<PathMut> {
        unsafe { CGPathCreateMutableCopyByTransformingPath(self, transform) }
    }

    #[inline]
    pub fn copy_dashing_path(
        &self,
        transform: Option<&cg::AffineTransform>,
        phase: cg::Float,
        lengths: &[cg::Float],
    ) -> arc::R<Self> {
        unsafe {
            CGPathCreateCopyByDashingPath(self, transform, phase, lengths.as_ptr(), lengths.len())
        }
    }

    #[inline]
    pub fn copy_stroking_path(
        &self,
        transform: Option<&cg::AffineTransform>,
        line_width: cg::Float,
        line_cap: cg::LineCap,
        line_join: cg::LineJoin,
        miter_limit: cg::Float,
    ) -> arc::R<Self> {
        unsafe {
            CGPathCreateCopyByStrokingPath(
                self,
                transform,
                line_width,
                line_cap,
                line_join,
                miter_limit,
            )
        }
    }

    #[inline]
    pub fn equal(&self, other: &Path) -> bool {
        unsafe { CGPathEqualToPath(self, other) }
    }

    #[inline]
    pub fn with_rect(rect: cg::Rect, transform: Option<&cg::AffineTransform>) -> arc::R<Self> {
        unsafe { CGPathCreateWithRect(rect, transform) }
    }

    #[inline]
    pub fn with_ellipse_in_rect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithEllipseInRect(rect, transform) }
    }

    #[inline]
    pub fn with_rounded_rect(
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Self> {
        unsafe { CGPathCreateWithRoundedRect(rect, corner_width, corner_height, transform) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CGPathIsEmpty(self) }
    }

    #[inline]
    pub fn is_rect(&self) -> bool {
        unsafe { CGPathIsRect(self) }
    }

    #[inline]
    pub fn current_point(&self) -> cg::Point {
        unsafe { CGPathGetCurrentPoint(self) }
    }

    #[inline]
    pub fn bounding_box(&self) -> cg::Rect {
        unsafe { CGPathGetBoundingBox(self) }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

define_cf_type!(PathMut(Path));
impl PathMut {
    pub fn new() -> arc::R<Self> {
        unsafe { CGPathCreateMutable() }
    }

    pub fn add_rounded_rect(
        &mut self,
        transform: Option<&cg::AffineTransform>,
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
    ) {
        unsafe { CGPathAddRoundedRect(self, transform, rect, corner_width, corner_height) }
    }

    #[inline]
    pub fn move_to_point(&mut self, m: Option<&cg::AffineTransform>, x: cg::Float, y: cg::Float) {
        unsafe { CGPathMoveToPoint(self, m, x, y) }
    }

    #[inline]
    pub fn add_line_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddLineToPoint(self, m, x, y) }
    }

    #[inline]
    pub fn add_quad_curve_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        cpx: cg::Float,
        cpy: cg::Float,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddQuadCurveToPoint(self, m, cpx, cpy, x, y) }
    }

    #[inline]
    pub fn add_curve_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        cp1x: cg::Float,
        cp1y: cg::Float,
        cp2x: cg::Float,
        cp2y: cg::Float,
        x: cg::Float,
        y: cg::Float,
    ) {
        unsafe { CGPathAddCurveToPoint(self, m, cp1x, cp1y, cp2x, cp2y, x, y) }
    }

    #[inline]
    pub fn close_subpath(&mut self) {
        unsafe { CGPathCloseSubpath(self) }
    }

    #[inline]
    pub fn add_rect(&mut self, m: Option<&cg::AffineTransform>, rect: cg::Rect) {
        unsafe { CGPathAddRect(self, m, rect) }
    }

    #[inline]
    pub fn add_rects(&mut self, m: Option<&cg::AffineTransform>, rects: &[cg::Rect]) {
        unsafe { CGPathAddRects(self, m, rects.as_ptr(), rects.len()) }
    }

    #[inline]
    pub fn add_lines(&mut self, m: Option<&cg::AffineTransform>, points: &[cg::Point]) {
        unsafe { CGPathAddLines(self, m, points.as_ptr(), points.len()) }
    }

    #[inline]
    pub fn add_ellipse_in_rect(&mut self, m: Option<&cg::AffineTransform>, rect: cg::Rect) {
        unsafe { CGPathAddEllipseInRect(self, m, rect) }
    }

    #[inline]
    pub fn add_relative_arc(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        delta: cg::Float,
    ) {
        unsafe { CGPathAddRelativeArc(self, m, x, y, radius, start_angle, delta) }
    }

    #[inline]
    pub fn add_arc(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        end_ange: cg::Float,
        clockwise: bool,
    ) {
        unsafe { CGPathAddArc(self, m, x, y, radius, start_angle, end_ange, clockwise) }
    }

    #[inline]
    pub fn add_arc_to_point(
        &mut self,
        m: Option<&cg::AffineTransform>,
        x1: cg::Float,
        y1: cg::Float,
        x2: cg::Float,
        y2: cg::Float,
        radius: cg::Float,
    ) {
        unsafe { CGPathAddArcToPoint(self, m, x1, y1, x2, y2, radius) }
    }

    #[inline]
    pub fn add_path(&mut self, m: Option<&cg::AffineTransform>, path2: &Path) {
        unsafe { CGPathAddPath(self, m, path2) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGPathGetTypeID() -> cf::TypeId;
    fn CGPathCreateMutable() -> arc::R<PathMut>;
    fn CGPathCreateCopy(path: &Path) -> arc::R<Path>;
    fn CGPathCreateMutableCopy(path: &Path) -> arc::R<PathMut>;
    fn CGPathCreateCopyByTransformingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;
    fn CGPathCreateMutableCopyByTransformingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<PathMut>;
    fn CGPathCreateWithRect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;
    fn CGPathCreateWithEllipseInRect(
        rect: cg::Rect,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;

    fn CGPathCreateWithRoundedRect(
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
        transform: Option<&cg::AffineTransform>,
    ) -> arc::R<Path>;

    fn CGPathAddRoundedRect(
        path: &mut PathMut,
        transform: Option<&cg::AffineTransform>,
        rect: cg::Rect,
        corner_width: cg::Float,
        corner_height: cg::Float,
    );

    fn CGPathCreateCopyByDashingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
        phase: cg::Float,
        lengths: *const cg::Float,
        count: usize,
    ) -> arc::R<Path>;

    fn CGPathCreateCopyByStrokingPath(
        path: &Path,
        transform: Option<&cg::AffineTransform>,
        line_width: cg::Float,
        line_cap: cg::LineCap,
        line_join: cg::LineJoin,
        miter_limit: cg::Float,
    ) -> arc::R<Path>;

    fn CGPathEqualToPath(path1: &Path, path2: &Path) -> bool;

    fn CGPathMoveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddLineToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddQuadCurveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        cpx: cg::Float,
        cpy: cg::Float,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathAddCurveToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        cp1x: cg::Float,
        cp1y: cg::Float,
        cp2x: cg::Float,
        cp2y: cg::Float,
        x: cg::Float,
        y: cg::Float,
    );

    fn CGPathCloseSubpath(path: &mut PathMut);

    fn CGPathAddRect(path: &mut PathMut, m: Option<&cg::AffineTransform>, rect: cg::Rect);

    fn CGPathAddRects(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        rects: *const cg::Rect,
        count: usize,
    );

    fn CGPathAddLines(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        rects: *const cg::Point,
        count: usize,
    );

    fn CGPathAddEllipseInRect(path: &mut PathMut, m: Option<&cg::AffineTransform>, rect: cg::Rect);

    fn CGPathAddRelativeArc(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        delta: cg::Float,
    );

    fn CGPathAddArc(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x: cg::Float,
        y: cg::Float,
        radius: cg::Float,
        start_angle: cg::Float,
        end_ange: cg::Float,
        clockwise: bool,
    );

    fn CGPathAddArcToPoint(
        path: &mut PathMut,
        m: Option<&cg::AffineTransform>,
        x1: cg::Float,
        y1: cg::Float,
        x2: cg::Float,
        y2: cg::Float,
        radius: cg::Float,
    );

    fn CGPathAddPath(path1: &mut PathMut, m: Option<&cg::AffineTransform>, path2: &Path);
    fn CGPathIsEmpty(path: &Path) -> bool;
    fn CGPathIsRect(path: &Path) -> bool;
    fn CGPathGetCurrentPoint(path: &Path) -> cg::Point;
    fn CGPathGetBoundingBox(path: &Path) -> cg::Rect;
}

#[cfg(test)]
mod tests {
    use crate::cg;

    #[test]
    fn basics() {
        let path = cg::Path::with_rect(cg::Rect::zero(), None);
        path.show();
        let path = cg::Path::with_ellipse_in_rect(cg::Rect::zero(), None);
        path.show();

        let path = path.copy_stroking_path(
            None,
            5.0f64,
            cg::LineCap::Round,
            cg::LineJoin::Round,
            0.0f64,
        );
        path.show();
    }
}
