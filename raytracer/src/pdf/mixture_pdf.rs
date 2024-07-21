// use crate::{pdf::Pdf, util::vec3::*};
// use rand::Rng;
//
// pub struct MixturePdf<'a,H1:Pdf,H2:Pdf> {
//     p: (&'a H1,&'a H2),
// }
//
// impl<'a,H1:Pdf,H2:Pdf> MixturePdf<'a,H1,H2> {
//     pub fn new(p0: &'a H1, p1: &'a H2) -> Self {
//         MixturePdf { p: (p0, p1) }
//     }
// }
//
// impl<'a,H1:Pdf,H2:Pdf> Pdf for MixturePdf<'a,H1,H2> {
//     fn value(&self, direction: &Vec3) -> f64 {
//         0.5 * self.p.0.value(direction) + 0.5 * self.p.1.value(direction)
//     }
//     fn generate(&self) -> Vec3 {
//         let mut rng = rand::thread_rng();
//         if rng.gen_range(0.0..1.0) < 0.5 {
//             self.p.0.generate()
//         } else {
//             self.p.1.generate()
//         }
//     }
// }
