use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3, Translation, Translation3};
use kiss3d::window::Window;

pub fn bruh() {
    let mut window = Window::new("Kiss3d: lines");

    window.set_light(Light::StickToCamera);
    let mut s = window.add_sphere(1.0);
    s.set_color(1.0, 0.0, 0.0);
    let t = Translation3::new(0.0, 2.0, 0.0);
    s.prepend_to_local_translation(&t);

    while window.render() {
        let a = Point3::new(-0.1, -0.1, 0.0);
        let b = Point3::new(0.0, 0.1, 0.0);
        let c = Point3::new(0.1, -0.1, 0.0);
        window.set_line_width(5.0);
        window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(&b, &c, &Point3::new(0.0, 1.0, 0.0));
        window.set_line_width(50.0);
        window.draw_line(&c, &a, &Point3::new(0.0, 0.0, 1.0));

        // window.draw_planar_line(
        //     &Point2::new(-100.0, -200.0),
        //     &Point2::new(100.0, -200.0),
        //     &Point3::new(1.0, 1.0, 1.0),
        // );
    }
}
