use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    _origin: Vec3,
    _direction: Vec3,
}

impl Ray {
    pub fn ray(origin:Vec3, direction:Vec3) -> Ray {
        Ray {_origin:origin, _direction:direction}
    }

    pub fn origin(self) -> Vec3 {
        self._origin
    }

    pub fn direction(self) -> Vec3 {
        self._direction
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self._origin + self._direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_origin() {
        let ray: Ray = Ray::ray(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.origin(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_direction() {

    }

    #[test]
    fn test_ray_point_at_parameter() {

    }
}
