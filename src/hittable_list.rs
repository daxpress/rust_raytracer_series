use std::rc::Rc;

use super::hittable::{Hittable, HitResult};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitResult> {
        let mut hit_record = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            let hit = object.hit(ray, ray_tmin, closest_so_far);
            match hit {
                None => (),
                Some(result) => {
                    closest_so_far = result.t();
                    hit_record = Some(result);
                }
            }
        }

        hit_record
    }
}