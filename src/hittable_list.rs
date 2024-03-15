use std::sync::Arc;

use super::hittable::{Hittable, HitResult};
use super::interval::Interval;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, interval: Interval) -> Option<HitResult> {
        let mut hit_record = None;
        let mut closest_so_far = interval.max();

        for object in &self.objects {
            let hit = object.hit(ray, Interval::new(interval.min(), closest_so_far));
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

unsafe impl Send for HittableList{}
unsafe impl Sync for HittableList{}