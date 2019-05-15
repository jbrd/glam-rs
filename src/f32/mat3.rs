use super::{
    super::Angle,
    {Quat, Vec3},
};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::ops::{Add, Mul, Sub};

pub fn mat3(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Mat3 {
    Mat3 {
        x_axis,
        y_axis,
        z_axis,
    }
}

#[inline]
fn quat_to_axes(rotation: Quat) -> (Vec3, Vec3, Vec3) {
    debug_assert!(rotation.is_normalized());
    let (x, y, z, w) = rotation.into();
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    let x_axis = Vec3::new(1.0 - (yy + zz), xy + wz, xz - wy);
    let y_axis = Vec3::new(xy - wz, 1.0 - (xx + zz), yz + wx);
    let z_axis = Vec3::new(xz + wy, yz - wx, 1.0 - (xx + yy));
    (x_axis, y_axis, z_axis)
}

#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
    pub(crate) x_axis: Vec3,
    pub(crate) y_axis: Vec3,
    pub(crate) z_axis: Vec3,
}

impl Mat3 {
    #[inline]
    pub fn zero() -> Self {
        Self {
            x_axis: Vec3::zero(),
            y_axis: Vec3::zero(),
            z_axis: Vec3::zero(),
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::unit_z(),
        }
    }

    #[inline]
    pub fn new(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline]
    pub fn from_scale_rotation(scale: Vec3, rotation: Quat) -> Self {
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        let (scale_x, scale_y, scale_z) = scale.into();
        Self {
            x_axis: x_axis * scale_x,
            y_axis: y_axis * scale_y,
            z_axis: z_axis * scale_z,
        }
    }

    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (x, y, z) = axis.into();
        let (xsin, ysin, zsin) = (axis * sin).into();
        let (x2, y2, z2) = (axis * axis).into();
        let omc = 1.0 - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self {
            x_axis: Vec3::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            y_axis: Vec3::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            z_axis: Vec3::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        }
    }

    #[inline]
    pub fn from_rotation_ypr(yaw: Angle, pitch: Angle, roll: Angle) -> Self {
        let quat = Quat::from_rotation_ypr(yaw, pitch, roll);
        Self::from_quat(quat)
    }

    #[inline]
    pub fn from_rotation_x(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::new(0.0, cosa, sina),
            z_axis: Vec3::new(0.0, -sina, cosa),
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec3::new(cosa, 0.0, -sina),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::new(sina, 0.0, cosa),
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec3::new(cosa, sina, 0.0),
            y_axis: Vec3::new(-sina, cosa, 0.0),
            z_axis: Vec3::unit_z(),
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        let (x, y, z) = scale.into();
        Self {
            x_axis: Vec3::new(x, 0.0, 0.0),
            y_axis: Vec3::new(0.0, y, 0.0),
            z_axis: Vec3::new(0.0, 0.0, z),
        }
    }

    #[inline]
    pub fn set_x_axis(&mut self, x: Vec3) {
        self.x_axis = x;
    }

    #[inline]
    pub fn set_y_axis(&mut self, y: Vec3) {
        self.y_axis = y;
    }

    #[inline]
    pub fn set_z_axis(&mut self, z: Vec3) {
        self.z_axis = z;
    }

    #[inline]
    pub fn get_x_axis(&self) -> Vec3 {
        self.x_axis
    }

    #[inline]
    pub fn get_y_axis(&self) -> Vec3 {
        self.y_axis
    }

    #[inline]
    pub fn get_z_axis(&self) -> Vec3 {
        self.z_axis
    }

    // #[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
    // #[inline]
    // pub fn transpose(&self) -> Self {
    //     // sse2 implemenation based off DirectXMath XMMatrixInverse (MIT License)
    //     #[cfg(target_arch = "x86")]
    //     use std::arch::x86::*;
    //     #[cfg(target_arch = "x86_64")]
    //     use std::arch::x86_64::*;

    //     unsafe {
    //         let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
    //         let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);
    //         let tmp2 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b01_00_01_00);
    //         let tmp3 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b11_10_11_10);

    //         Self {
    //             x_axis: _mm_shuffle_ps(tmp0, tmp2, 0b10_00_10_00).into(),
    //             y_axis: _mm_shuffle_ps(tmp0, tmp2, 0b11_01_11_01).into(),
    //             z_axis: _mm_shuffle_ps(tmp1, tmp3, 0b10_00_10_00).into(),
    //         }
    //     }
    // }

    // #[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
    #[inline]
    pub fn transpose(&self) -> Self {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        Self {
            x_axis: Vec3::new(m00, m10, m20),
            y_axis: Vec3::new(m01, m11, m21),
            z_axis: Vec3::new(m02, m12, m22),
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        self.x_axis.dot(Vec3::new(
            m11 * m22 - m12 * m21,
            m12 * m20 - m10 * m22,
            m10 * m21 - m11 * m20,
        ))
    }

    // #[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
    pub fn inverse(&self) -> Self {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        let inv00 = m11 * m22 - m12 * m21;
        let inv10 = m12 * m20 - m10 * m22;
        let inv20 = m10 * m21 - m11 * m20;
        let dot = self.x_axis.dot(Vec3::new(inv00, inv10, inv20));
        debug_assert!(dot.abs() > 0.000001);
        let inv_dot = Vec3::splat(1.0 / dot);

        Mat3 {
            x_axis: inv_dot * Vec3::new(inv00, m02 * m21 - m01 * m22, m01 * m12 - m02 * m11),
            y_axis: inv_dot * Vec3::new(inv10, m00 * m22 - m02 * m20, m02 * m10 - m00 * m12),
            z_axis: inv_dot * Vec3::new(inv20, m01 * m20 - m00 * m21, m00 * m11 - m01 * m10),
        }
    }

    // #[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
    // /// Performs a matrix inverse. Note that this method does not check if the matrix is
    // /// invertible and will divide by zero if a non-invertible matrix is inverted.
    // pub fn inverse(&self) -> Self {
    //     // sse2 implemenation based off DirectXMath XMMatrixInverse (MIT License)
    //     #[cfg(target_arch = "x86")]
    //     use std::arch::x86::*;
    //     #[cfg(target_arch = "x86_64")]
    //     use std::arch::x86_64::*;

    //     macro_rules! permute {
    //         ($v:expr, $mask:expr) => {
    //             _mm_shuffle_ps($v, $v, $mask)
    //         };
    //     };

    //     macro_rules! _MM_SHUFFLE {
    //         ($z:expr, $y:expr, $x:expr, $w:expr) => {
    //             ($z << 6) | ($y << 4) | ($x << 2) | $w
    //         };
    //     };

    //     macro_rules! neg_mul_sub {
    //         ($a:expr, $b:expr, $c:expr) => {
    //             _mm_sub_ps($c, _mm_mul_ps($a, $b))
    //         };
    //     };

    //     macro_rules! mul_add {
    //         ($a:expr, $b:expr, $c:expr) => {
    //             _mm_add_ps(_mm_mul_ps($a, $b), $c)
    //         };
    //     };

    //     let mt = self.transpose();
    //     let mtx = mt.get_x_axis().into();
    //     let mty = mt.get_y_axis().into();
    //     let mtz = mt.get_z_axis().into();
    //     let mtw = mt.get_w_axis().into();

    //     unsafe {
    //         let mut v00 = permute!(mtz, _MM_SHUFFLE!(1, 1, 0, 0));
    //         let mut v10 = permute!(mtw, _MM_SHUFFLE!(3, 2, 3, 2));
    //         let mut v01 = permute!(mtx, _MM_SHUFFLE!(1, 1, 0, 0));
    //         let mut v11 = permute!(mty, _MM_SHUFFLE!(3, 2, 3, 2));
    //         let mut v02 = _mm_shuffle_ps(mtz, mtx, _MM_SHUFFLE!(2, 0, 2, 0));
    //         let mut v12 = _mm_shuffle_ps(mtw, mty, _MM_SHUFFLE!(3, 1, 3, 1));

    //         let mut d0 = _mm_mul_ps(v00, v10);
    //         let mut d1 = _mm_mul_ps(v01, v11);
    //         let mut d2 = _mm_mul_ps(v02, v12);

    //         v00 = permute!(mtz, _MM_SHUFFLE!(3, 2, 3, 2));
    //         v10 = permute!(mtw, _MM_SHUFFLE!(1, 1, 0, 0));
    //         v01 = permute!(mtx, _MM_SHUFFLE!(3, 2, 3, 2));
    //         v11 = permute!(mty, _MM_SHUFFLE!(1, 1, 0, 0));
    //         v02 = _mm_shuffle_ps(mtz, mtx, _MM_SHUFFLE!(3, 1, 3, 1));
    //         v12 = _mm_shuffle_ps(mtw, mty, _MM_SHUFFLE!(2, 0, 2, 0));

    //         // v00 = _mm_mul_ps(v00 * v10);
    //         // v01 = _mm_mul_ps(v01 * v11);
    //         // v02 = _mm_mul_ps(v02 * v12);
    //         // d0 = _mm_sub_ps(d0,v00);
    //         // d1 = _mm_sub_ps(d1,v01);
    //         // d2 = _mm_sub_ps(d2,v02);
    //         d0 = neg_mul_sub!(v00, v10, d0);
    //         d1 = neg_mul_sub!(v01, v11, d1);
    //         d2 = neg_mul_sub!(v02, v12, d2);

    //         // V11 = D0Y,D0W,D2Y,D2Y
    //         v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 1, 3, 1));
    //         v00 = permute!(mty, _MM_SHUFFLE!(1, 0, 2, 1));
    //         v10 = _mm_shuffle_ps(v11, d0, _MM_SHUFFLE!(0, 3, 0, 2));
    //         v01 = permute!(mtx, _MM_SHUFFLE!(0, 1, 0, 2));
    //         v11 = _mm_shuffle_ps(v11, d0, _MM_SHUFFLE!(2, 1, 2, 1));

    //         // V13 = D1Y,D1W,D2W,D2W
    //         let mut v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 3, 3, 1));
    //         v02 = permute!(mtw, _MM_SHUFFLE!(1, 0, 2, 1));
    //         v12 = _mm_shuffle_ps(v13, d1, _MM_SHUFFLE!(0, 3, 0, 2));
    //         let mut v03 = permute!(mtz, _MM_SHUFFLE!(0, 1, 0, 2));
    //         v13 = _mm_shuffle_ps(v13, d1, _MM_SHUFFLE!(2, 1, 2, 1));

    //         let mut c0 = _mm_mul_ps(v00, v10);
    //         let mut c2 = _mm_mul_ps(v01, v11);
    //         let mut c4 = _mm_mul_ps(v02, v12);
    //         let mut c6 = _mm_mul_ps(v03, v13);

    //         // V11 = D0X,D0Y,D2X,D2X
    //         v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(0, 0, 1, 0));
    //         v10 = _mm_shuffle_ps(d0, v11, _MM_SHUFFLE!(2, 1, 0, 3));
    //         v01 = permute!(mtx, _MM_SHUFFLE!(1, 3, 2, 3));
    //         v11 = _mm_shuffle_ps(d0, v11, _MM_SHUFFLE!(0, 2, 1, 2));

    //         // V13 = D1X,D1Y,D2Z,D2Z
    //         v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(2, 2, 1, 0));
    //         v02 = permute!(mtw, _MM_SHUFFLE!(2, 1, 3, 2));
    //         v12 = _mm_shuffle_ps(d1, v13, _MM_SHUFFLE!(2, 1, 0, 3));
    //         v03 = permute!(mtz, _MM_SHUFFLE!(1, 3, 2, 3));
    //         v13 = _mm_shuffle_ps(d1, v13, _MM_SHUFFLE!(0, 2, 1, 2));

    //         // V00 = _mm_mul_ps(V00,V10);
    //         // V01 = _mm_mul_ps(V01,V11);
    //         // V02 = _mm_mul_ps(V02,V12);
    //         // V03 = _mm_mul_ps(V03,V13);
    //         // C0 = _mm_sub_ps(C0,V00);
    //         // C2 = _mm_sub_ps(C2,V01);
    //         // C4 = _mm_sub_ps(C4,V02);
    //         // C6 = _mm_sub_ps(C6,V03);
    //         c0 = neg_mul_sub!(v00, v10, c0);
    //         c2 = neg_mul_sub!(v01, v11, c2);
    //         c4 = neg_mul_sub!(v02, v12, c4);
    //         c6 = neg_mul_sub!(v03, v13, c6);

    //         v00 = permute!(mty, _MM_SHUFFLE!(0, 3, 0, 3));
    //         // V10 = D0Z,D0Z,D2X,D2Y
    //         v10 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 0, 2, 2));
    //         v10 = permute!(v10, _MM_SHUFFLE!(0, 2, 3, 0));
    //         v01 = permute!(mtx, _MM_SHUFFLE!(2, 0, 3, 1));
    //         // V11 = D0X,D0W,D2X,D2Y
    //         v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 0, 3, 0));
    //         v11 = permute!(v11, _MM_SHUFFLE!(2, 1, 0, 3));
    //         v02 = permute!(mtw, _MM_SHUFFLE!(0, 3, 0, 3));
    //         // V12 = D1Z,D1Z,D2Z,D2W
    //         v12 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 2, 2, 2));
    //         v12 = permute!(v12, _MM_SHUFFLE!(0, 2, 3, 0));
    //         v03 = permute!(mtz, _MM_SHUFFLE!(2, 0, 3, 1));
    //         // V13 = D1X,D1W,D2Z,D2W
    //         v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 2, 3, 0));
    //         v13 = permute!(v13, _MM_SHUFFLE!(2, 1, 0, 3));

    //         // V00 = _mm_mul_ps(V00,V10);
    //         // V01 = _mm_mul_ps(V01,V11);
    //         // V02 = _mm_mul_ps(V02,V12);
    //         // V03 = _mm_mul_ps(V03,V13);
    //         // XMVECTOR C1 = _mm_sub_ps(C0,V00);
    //         // C0 = _mm_add_ps(C0,V00);
    //         // XMVECTOR C3 = _mm_add_ps(C2,V01);
    //         // C2 = _mm_sub_ps(C2,V01);
    //         // XMVECTOR C5 = _mm_sub_ps(C4,V02);
    //         // C4 = _mm_add_ps(C4,V02);
    //         // XMVECTOR C7 = _mm_add_ps(C6,V03);
    //         // C6 = _mm_sub_ps(C6,V03);
    //         let c1 = neg_mul_sub!(v00, v10, c0);
    //         c0 = mul_add!(v00, v10, c0);
    //         let c3 = mul_add!(v01, v11, c2);
    //         c2 = neg_mul_sub!(v01, v11, c2);
    //         let c5 = neg_mul_sub!(v02, v12, c4);
    //         c4 = mul_add!(v02, v12, c4);
    //         let c7 = mul_add!(v03, v13, c6);
    //         c6 = neg_mul_sub!(v03, v13, c6);

    //         c0 = _mm_shuffle_ps(c0, c1, _MM_SHUFFLE!(3, 1, 2, 0));
    //         c2 = _mm_shuffle_ps(c2, c3, _MM_SHUFFLE!(3, 1, 2, 0));
    //         c4 = _mm_shuffle_ps(c4, c5, _MM_SHUFFLE!(3, 1, 2, 0));
    //         c6 = _mm_shuffle_ps(c6, c7, _MM_SHUFFLE!(3, 1, 2, 0));
    //         let c0 = Vec4(permute!(c0, _MM_SHUFFLE!(3, 1, 2, 0)));
    //         let c2 = Vec4(permute!(c2, _MM_SHUFFLE!(3, 1, 2, 0)));
    //         let c4 = Vec4(permute!(c4, _MM_SHUFFLE!(3, 1, 2, 0)));
    //         let c6 = Vec4(permute!(c6, _MM_SHUFFLE!(3, 1, 2, 0)));

    //         // Get the determinate
    //         // XMVECTOR vTemp = XMVector4Dot(C0,mtx);
    //         let det = Vec4::splat(c0.dot(Vec4(mtx)));
    //         // if (pDeterminant != nullptr)
    //         //     *pDeterminant = vTemp;
    //         // vTemp = _mm_div_ps(g_XMOne,vTemp);
    //         let inv_det = Vec4::splat(1.0) / det;

    //         // XMMATRIX mResult;
    //         // mResult.r[0] = _mm_mul_ps(C0,vTemp);
    //         // mResult.r[1] = _mm_mul_ps(C2,vTemp);
    //         // mResult.r[2] = _mm_mul_ps(C4,vTemp);
    //         // mResult.r[3] = _mm_mul_ps(C6,vTemp);
    //         // return mResult;
    //         Self {
    //             x_axis: c0 * inv_det,
    //             y_axis: c2 * inv_det,
    //             z_axis: c4 * inv_det,
    //             w_axis: c6 * inv_det,
    //         }
    //     }
    // }

    #[inline]
    /// Multiplies two 3x3 matrices.
    /// Multiplication order is as follows:
    /// `local_to_world = local_to_object * local_to_world`
    pub fn mul_mat3(&self, rhs: &Self) -> Self {
        let mut tmp = self.x_axis.dup_x().mul(rhs.x_axis);
        tmp = self.x_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.x_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let x_axis = tmp;

        tmp = self.y_axis.dup_x().mul(rhs.x_axis);
        tmp = self.y_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.y_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let y_axis = tmp;

        tmp = self.z_axis.dup_x().mul(rhs.x_axis);
        tmp = self.z_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.z_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let z_axis = tmp;

        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline]
    pub fn add_mat3(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
        }
    }

    #[inline]
    pub fn sub_mat3(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
        }
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let s = Vec3::splat(rhs);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
            z_axis: self.z_axis * s,
        }
    }
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec3 {
    #[inline]
    /// Multiplies a 3x3 matrix and a 3D vector.
    /// Multiplication order is as follows:
    /// `world_direction = local_direction.transform_mat3(local_to_world)`
    pub fn transform_mat3(self, rhs: &Mat3) -> Self {
        let mut tmp = self.dup_x().mul(rhs.get_x_axis());
        tmp = self.dup_y().mul_add(rhs.get_y_axis(), tmp);
        tmp = self.dup_z().mul_add(rhs.get_z_axis(), tmp);
        tmp
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat3 {
        rng.gen::<[[f32; 3]; 3]>().into()
    }
}

impl Add<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.add_mat3(&rhs)
    }
}

impl Add<&Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add_mat3(rhs)
    }
}

impl Sub<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub_mat3(&rhs)
    }
}

impl Sub<&Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub_mat3(rhs)
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat3(&rhs)
    }
}

impl Mul<&Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul_mat3(rhs)
    }
}

impl Mul<Mat3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Mat3) -> Vec3 {
        self.transform_mat3(&rhs)
    }
}

impl Mul<&Mat3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Mat3) -> Vec3 {
        self.transform_mat3(rhs)
    }
}

impl Mul<Mat3> for f32 {
    type Output = Mat3;
    #[inline]
    fn mul(self, rhs: Mat3) -> Mat3 {
        rhs.mul_scalar(self)
    }
}
impl Mul<&Mat3> for f32 {
    type Output = Mat3;
    #[inline]
    fn mul(self, rhs: &Mat3) -> Mat3 {
        rhs.mul_scalar(self)
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl PartialEq for Mat3 {
    #[inline]
    fn eq(&self, rhs: &Mat3) -> bool {
        self.x_axis == rhs.x_axis && self.y_axis == rhs.y_axis && self.z_axis == rhs.z_axis
    }
}

impl From<[[f32; 3]; 3]> for Mat3 {
    #[inline]
    fn from(m: [[f32; 3]; 3]) -> Self {
        Mat3 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
        }
    }
}

impl From<&Mat3> for [[f32; 3]; 3] {
    #[inline]
    fn from(m: &Mat3) -> Self {
        [m.x_axis.into(), m.y_axis.into(), m.z_axis.into()]
    }
}

impl From<&[[f32; 3]; 3]> for Mat3 {
    #[inline]
    fn from(m: &[[f32; 3]; 3]) -> Self {
        Mat3 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
        }
    }
}

impl From<Mat3> for [[f32; 3]; 3] {
    #[inline]
    fn from(m: Mat3) -> Self {
        [m.x_axis.into(), m.y_axis.into(), m.z_axis.into()]
    }
}