

struct CollisionData {};
trait CollidesWith<T> {
  fn collides(&self, HitboxType) -> CollisionData;
}

struct Circle;
struct Box;
struct ConvexPolygon;

impl CollidesWith<Circle> for Circle {
  fn collides(&self, circle: Circle) -> CollisionData {
    CollisionData {}
  }
}
impl CollidesWith<Box> for Circle {
  fn collides(&self, box: Box) -> CollisionData {
    CollisionData {}
  }
}
impl CollidesWith<ConvexPolygon> for Circle {
  fn collides(&self, box: ConvexPolygon) -> CollisionData {
    CollisionData {}
  }
}