use crate::utils::aabb::Aabb;
// use crate::utils::bvh::BvhNode;
use crate::utils::color::Color;
use crate::utils::flats::Tria;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::material::Lambertian;
use crate::utils::normal_map::OriginMap;
use crate::utils::ray::Ray;
use crate::utils::vec3::{cross, dot, Point3, Vec3};
use image::RgbImage;
use rand::Rng;
use std::path::PathBuf;
use std::sync::Arc;
use tobj;

type TriaCoord = ((f64, f64), (f64, f64), (f64, f64));
pub struct Mesh {
    objects: Vec<Vec<Tria>>,                //obj<mesh<Triangles>>
    tex_coords: Vec<(i32, Vec<TriaCoord>)>, // usize is the index in tex_pic
    tex_pic: Vec<RgbImage>,
    bbox: Aabb,
} // only accept triangular meshes

impl Mesh {
    pub fn new(obj_path: &str, scale: f64) -> Self {
        let load_result = tobj::load_obj(obj_path, &tobj::GPU_LOAD_OPTIONS);
        assert!(load_result.is_ok());

        let mut par_dir = PathBuf::from(obj_path);
        let par_dir: &str = if let Some(par) = par_dir.parent() {
            par_dir = par.to_path_buf();
            par_dir.push("");
            par_dir.to_str().unwrap()
        } else {
            ""
        }; // parent directory location

        let nmap = Arc::new(OriginMap::default());
        let origin_mat = Arc::new(Lambertian::new_color(Color::new(1.0, 1.0, 1.0)));

        let (models, materials) = load_result.expect("Fail to load this .obj file");
        let materials = materials.expect("Fail to lod this .mtl file");

        let mut objects: Vec<Vec<Tria>> = Vec::new();
        let mut tex_coords: Vec<(i32, Vec<TriaCoord>)> = Vec::new();
        let mut tex_pic: Vec<RgbImage> = Vec::new();
        let mut bbox = Aabb::default();
        let mut tex_index: i32 = 0;

        for (_, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut object: Vec<Tria> = Vec::new();
            let mut tex_coord: Vec<TriaCoord> = Vec::new();
            // determine the mat here
            let mat_pic: Option<RgbImage> = if let Some(mat_id) = mesh.material_id {
                if let Some(pic_path) = &materials[mat_id].diffuse_texture {
                    let pic_path = par_dir.to_string() + pic_path;
                    let pic = image::open(pic_path).unwrap().to_rgb8();
                    Some(pic)
                } else {
                    None
                }
            } else {
                None
            };
            // must have 3 points, 9 f64s
            assert_eq!(mesh.positions.len() % 9, 0);
            for v in 0..mesh.positions.len() / 9 {
                let (p0, p1, p2) = (
                    Point3::new(
                        mesh.positions[9 * v] as f64,
                        mesh.positions[9 * v + 1] as f64,
                        mesh.positions[9 * v + 2] as f64,
                    ) * scale,
                    Point3::new(
                        mesh.positions[9 * v + 3] as f64,
                        mesh.positions[9 * v + 4] as f64,
                        mesh.positions[9 * v + 5] as f64,
                    ) * scale,
                    Point3::new(
                        mesh.positions[9 * v + 6] as f64,
                        mesh.positions[9 * v + 7] as f64,
                        mesh.positions[9 * v + 8] as f64,
                    ) * scale,
                );
                let tria = Tria::new_point(p0, p1, p2, origin_mat.clone(), nmap.clone());
                bbox = Aabb::new_aabb(&bbox, tria.bounding_box());
                object.push(tria);
            }

            let mtl_id = if mat_pic.is_some() {
                let tmp = tex_index;
                tex_index += 1;
                tmp
            } else {
                -1
            };

            if mat_pic.is_some() {
                // read vt
                assert_eq!(mesh.texcoords.len() % 6, 0);
                for v in 0..mesh.texcoords.len() / 6 {
                    let coord: ((f64, f64), (f64, f64), (f64, f64)) = (
                        (
                            mesh.texcoords[v * 6] as f64,
                            mesh.texcoords[v * 6 + 1] as f64,
                        ),
                        (
                            mesh.texcoords[v * 6 + 2] as f64,
                            mesh.texcoords[v * 6 + 3] as f64,
                        ),
                        (
                            mesh.texcoords[v * 6 + 4] as f64,
                            mesh.texcoords[v * 6 + 5] as f64,
                        ),
                    );
                    tex_coord.push(coord);
                }
            }

            if let Some(pic_mat) = mat_pic {
                tex_pic.push(pic_mat);
            }

            objects.push(object);
            tex_coords.push((mtl_id, tex_coord));
        }
        Mesh {
            objects,
            tex_coords,
            tex_pic,
            bbox,
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r,ray_t){
            return None;
        }
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for (i, oobject) in self.objects.iter().enumerate() {
            for (j, object) in oobject.iter().enumerate() {
                if let Some(mut temp_rec) = object.hit(r, &Interval::new(ray_t.min, closest_so_far))
                {
                    closest_so_far = temp_rec.t;
                    // change color here
                    // if self.tex_coords[i].0 >= 0 {
                    //     let pic = &self.tex_pic[self.tex_coords[i].0 as usize];
                    //     let (q, p, r) = &self.tex_coords[i].1[j];
                    //     // respectively q,p,r
                    //     let phv = temp_rec.p - object.q;
                    //     let alpha = dot(&object.w, &cross(&phv, &object.v)); //q->p
                    //     let beta = dot(&object.w, &cross(&object.u, &phv)); //q->r
                    // 
                    //     let pic_pos = (
                    //         q.0 + alpha * (p.0 - q.0) + beta * (r.0 - q.0),
                    //         1.0 - (q.1 + alpha * (p.1 - q.1) + beta * (r.1 - q.1)),
                    //     );
                    //     let (w, h) = pic.dimensions();
                    //     let (x, y) = ((w as f64 * pic_pos.0) as u32, (h as f64 * pic_pos.1) as u32);
                    //     let pic_color = pic.get_pixel(x, y);
                    //     let color = Color::new(
                    //         pic_color.0[0] as f64 / 255.999,
                    //         pic_color.0[1] as f64 / 255.999,
                    //         pic_color.0[2] as f64 / 255.999,
                    //     );
                    //     temp_rec.mat = Arc::new(Lambertian::new_color(color));
                    // }
                    rec = Some(temp_rec);
                }
            }
        }
        rec
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut len: usize = 0;
        for oobject in &self.objects {
            len += oobject.len();
        }
        let weight = 1.0 / len as f64;
        let mut sum = 0.0;

        for oobject in &self.objects {
            for object in oobject {
                sum += weight * object.pdf_value(origin, direction);
            }
        }
        sum
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let ppos = rng.gen_range(0..self.objects.len());
        let pos = rng.gen_range(0..self.objects[ppos].len());
        self.objects[ppos][pos].random(origin)
    }
}
