
use crate::{
    vec3,
    vec3::Vec3
};

#[test]
pub fn create_vect() {
    {
        let v1 = Vec3::new(0.5, 1.8, 12.0);
        assert_eq!(0.5, v1.x());
        assert_eq!(1.8, v1.y());
        assert_eq!(12.0, v1.z());
    }
}

#[test]
pub fn add_vect() {
    {
        let v1 = Vec3::new(0.5, 0.0, 0.0);
        let v2 = Vec3::new(1, 0, 0);
        let ret = Vec3::new(1.5, 0.0, 0.0);
        assert_eq!(ret, v1 + v2);
    }
    {
        let v1 = Vec3::new(0.5, 1.8, 12.0);
        let v2 = Vec3::new(1.0, 0.3, -4.0);
        let ret = Vec3::new(1.5, 2.1, 8.0);
        assert_eq!(ret, v1 + v2);
    }
    {
        let v1 = Vec3::new(0.5, 1.8, 12.0);
        let ret = Vec3::new(1.5, 2.1, 8.0);
        assert_eq!(ret, v1 + [1.0, 0.3, -4.0]);
    }
}
#[test]
pub fn sub_vect() {
    {
        let v1 = Vec3::new(0.5, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let ret = Vec3::new(-0.5, 0.0, 0.0);
        assert_eq!(ret, v1 - v2);
    }
    {
        let v1 = Vec3::new(0.5, 1.8, 12.0);
        let v2 = Vec3::new(1.0, 0.3, -4.0);
        let ret = Vec3::new(-0.5, 1.5, 16.0);
        assert_eq!(ret, v1 - v2);
    }
    {
        let v1 = Vec3::new(0.5, 1.8, 12.0);
        let v2 = Vec3::new(1.0, 0.3, -4.0);
        let ret = Vec3::new(-0.5, 1.5, 16.0);
        assert_eq!(ret, v1 - v2);
        assert_eq!(ret, v1 - [1.0, 0.3, -4.0]);
    }
}

#[test]
pub fn magnitude() {
    {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let ret = 1.0;
        assert_eq!(v.magnitude(), ret);
    }
    {
        let v = Vec3::new(0.0, 1.0, 0.0);
        let ret = 1.0;
        assert_eq!(v.magnitude(), ret);
    }
    {
        let v = Vec3::new(0.0, 0.0, 1.0);
        let ret = 1.0;
        assert_eq!(v.magnitude(), ret);
    }
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let ret = (14 as f64).sqrt();
        assert_eq!(v.magnitude(), ret);
    }
    {
        let v = Vec3::new(-1.0, -2.0, -3.0);
        let ret = (14 as f64).sqrt();
        assert_eq!(v.magnitude(), ret);
    }
}

#[test]
pub fn scale() {
    {
        let a = Vec3::new(1.0, -2.0, 3.0);
        let ret = Vec3::new(3.5, -7.0, 10.5,);
        assert_eq!(a.scale(3.5), ret);
    }
    {
        let a = Vec3::new(1.0, -2.0, 3.0);
        let ret = Vec3::new(0.5, -1.0, 1.5);
        assert_eq!(a.scale(0.5), ret);
    }
    {
        let a = Vec3::new(1.0, -2.0, 3.0);
        let ret = Vec3::new(0.5, -1.0, 1.5);
        assert_eq!(a.scale(1.0 / 2.0), ret);
    }
}

#[test]
pub fn multiply() {
    let a = Vec3::new(18.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 12.0, 11.0);
    let ans = Vec3::new(18.0, 24.0, 33.0);
    let ret0 = a* [1.0, 12.0, 11.0];
    let ret = a*b;
    assert_eq!(ret, ans);
    assert_eq!(ret0, ans);
}

#[test]
pub fn dot_product() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(20.0, a.dot(b));
}

#[test]
pub fn cross_product() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let ret1 = Vec3::new(-1.0, 2.0, -1.0);
    let ret2 = Vec3::new(1.0, -2.0, 1.0);
    assert_eq!(a.cross(b), ret1);
    assert_eq!(b.cross(a), ret2);
}

#[test]
pub fn test_from() {
    let v: Vec3 = [1.0, 2.0, 3.0].into();
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v, v2);
}

#[test]
pub fn unit_vector() {
    {
        let v: Vec3 = [4.0, 0.0, 0.0].into();
        let ret: Vec3 = [1.0, 0.0, 0.0].into();
        assert_eq!(v.unit_vector(), ret);
    }
    {
        let div = (14 as f64).sqrt();
        let v = Vec3::new(1.0, 2.0, 3.0);
        let ret = Vec3::new(1.0 / div, 2.0 / div, 3.0 / div);
        assert_eq!(v.unit_vector(), ret);
    }
}