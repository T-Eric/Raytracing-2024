use crate::{
    hittable::{bvh::BvhNode, hittable_list::HittableList, instances::flats::Tria, Hittable},
    materials::lambertian::Lambertian,
    textures::image_texture::ImageTexture,
    util::vec3::*,
};
use std::ops::Deref;
use std::path::Path;
use tobj;

pub struct LoadParam<'a> {
    pub obj_file: &'a str,
    pub zoom: f64,
    pub offset: Vec3,
    pub rot_x: f64,
    pub rot_y: f64, //degree
    pub rot_z: f64,
}

pub fn obj_mesh(param: LoadParam) -> Box<dyn Hittable> {
    let the_obj = tobj::load_obj(param.obj_file, &tobj::GPU_LOAD_OPTIONS);
    assert!(the_obj.is_ok());
    let (models, materials) = the_obj.unwrap();
    let materials = materials.unwrap();
    let dir = Path::new(param.obj_file).parent();
    let dir = if let Some(par) = dir {
        let mut par = String::from(par.to_str().unwrap());
        if !par.ends_with('/') {
            par.push('/');
        }
        par
    } else {
        String::from("")
    };
    let dir = dir.deref();
    let mut trias = Vec::<Tria<_>>::new();

    for (_, md) in models.iter().enumerate() {
        let mesh = &md.mesh;
        let mut points = Vec::<_>::new();
        let mut texs = Vec::<_>::new();
        let mut normals = Vec::<_>::new();
        let mat_id = md.mesh.material_id.unwrap();
        let mat_file =
            String::from(dir) + materials[mat_id].diffuse_texture.clone().unwrap().as_str();
        let mat_pic = ImageTexture::new_path(mat_file.as_str());
        let mat = Lambertian::new_tex(mat_pic);
        assert_eq!(mesh.positions.len() % 3, 0);
        assert_ne!(mesh.texcoords.len(), 0);
        assert_ne!(mesh.normals.len(), 0);
        assert_ne!(mesh.indices.len(), 0);

        for p in mesh.positions.chunks(3) {
            points.push(Point3::new(p[0] as f64, p[1] as f64, p[2] as f64));
        }
        for t in mesh.texcoords.chunks(2) {
            texs.push((t[0] as f64, t[1] as f64));
        }
        for n in mesh.normals.chunks(3) {
            normals.push(Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64));
        }

        for id in mesh.indices.chunks(3) {
            let mut tria = Tria::new(
                &points,
                &texs,
                &normals,
                [id[0] as usize, id[1] as usize, id[2] as usize],
                mat.clone(),
            );
            tria.zoom(param.zoom);
            tria.trans(param.offset);
            trias.push(tria);
        }
    }

    let mut world = HittableList::default();
    let mut center_old = Vec3::default();
    let tot_points: f64 = 3.0 * trias.len() as f64;
    for t in trias.iter() {
        for i in 0..3 {
            center_old.e[0] += t.p[i].e[0] / tot_points;
            center_old.e[1] += t.p[i].e[1] / tot_points;
            center_old.e[2] += t.p[i].e[2] / tot_points;
        }
    }
    for t in trias {
        let mut r_tri = t;
        r_tri.rotate(center_old, param.rot_x, param.rot_y, param.rot_z);
        world.add(Box::new(r_tri));
    }
    Box::new(BvhNode::new_list(world))
}
