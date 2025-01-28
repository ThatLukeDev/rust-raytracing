#[cfg(test)]
mod vector {
    use crate::vector::*;

    #[test]
    fn add() {
        assert_eq!(Vec3::new(2,4,6) + Vec3::new(3,6,9), Vec3::new(5,10,15));
        assert_eq!(Vec3::new(3,9,3) + Vec3::new(2,7,1), Vec3::new(5,16,4));
        assert_eq!(Vec3::new(5,9,5) + Vec3::new(5,4,2), Vec3::new(10,13,7));
        assert_eq!(Vec3::new(7,8,5) + Vec3::new(3,3,3), Vec3::new(10,11,8));
        assert_eq!(Vec3::new(0,1,2) + Vec3::new(1,2,4), Vec3::new(1,3,6));
    }

    #[test]
    fn sub() {
        assert_eq!(Vec3::new(2,4,6) - Vec3::new(3,6,9), Vec3::new(-1,-2,-3));
        assert_eq!(Vec3::new(3,9,3) - Vec3::new(2,7,1), Vec3::new(1,2,2));
        assert_eq!(Vec3::new(5,9,5) - Vec3::new(5,4,2), Vec3::new(0,5,3));
        assert_eq!(Vec3::new(7,8,5) - Vec3::new(3,3,3), Vec3::new(4,5,2));
        assert_eq!(Vec3::new(0,1,2) - Vec3::new(1,2,4), Vec3::new(-1,-1,-2));
    }

    #[test]
    fn scale() {
        assert_eq!(Vec3::new(2,4,6) * 3, Vec3::new(6,12,18));
        assert_eq!(Vec3::new(3,9,3) * 2, Vec3::new(6,18,6));
        assert_eq!(Vec3::new(5,9,5) * 5, Vec3::new(25,45,25));
        assert_eq!(Vec3::new(7,8,5) * 3, Vec3::new(21,24,15));
        assert_eq!(Vec3::new(0,1,2) * 1, Vec3::new(0,1,2));
    }

    #[test]
    fn overscale() {
        assert_eq!(Vec3::new(2.0,4.0,6.0) / 4.0, Vec3::new(0.5,1.0,1.5));
        assert_eq!(Vec3::new(3.0,9.0,3.0) / 2.0, Vec3::new(1.5,4.5,1.5));
        assert_eq!(Vec3::new(5.0,9.0,5.0) / 5.0, Vec3::new(1.0,1.8,1.0));
        assert_eq!(Vec3::new(7.0,8.0,5.0) / 10.0, Vec3::new(0.7,0.8,0.5));
        assert_eq!(Vec3::new(0.0,1.0,2.0) / 1.0, Vec3::new(0.0,1.0,2.0));
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::new(2,4,6) * Vec3::new(3,6,9), 84);
        assert_eq!(Vec3::new(3,9,3) * Vec3::new(2,7,1), 72);
        assert_eq!(Vec3::new(5,9,5) * Vec3::new(5,4,2), 71);
        assert_eq!(Vec3::new(7,8,5) * Vec3::new(3,3,3), 60);
        assert_eq!(Vec3::new(0,1,2) * Vec3::new(1,2,4), 10);
    }

    #[test]
    fn unit() {
        assert_eq!(Vec3::new(2.0,0.0,0.0).unit(), Vec3::new(1.0,0.0,0.0));
        assert_eq!(Vec3::new(0.0,5.0,0.0).unit(), Vec3::new(0.0,1.0,0.0));
        assert_eq!(Vec3::new(0.0,0.0,4.0).unit(), Vec3::new(0.0,0.0,1.0));
    }
}

#[cfg(test)]
mod ray {
    use crate::vector::*;
    use crate::ray::*;

    #[test]
    fn at() {
        assert_eq!(
            Ray::new(Vec3::new(1.0,2.0,3.0),Vec3::new(1.0,0.0,0.0)).at(0),
            Vec3::new(1.0,2.0,3.0)
        );
    }
}

#[cfg(test)]
mod sphere {
    mod raytrace {
        use crate::vector::*;
        use crate::ray::*;
        use crate::raytrace::*;
        use crate::sphere::*;

        #[test]
        fn intersects() {
            assert_eq!(
                Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0).intersects_at(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))),
                Some(Vec3::new(0.0, 1.0, 0.0))
            );
            assert_eq!(
                Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0).intersects_along(&Ray::new(Vec3::new(3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
                Some(4.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0).intersects_along(&Ray::new(Vec3::new(-3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
                None
            );
        }

        #[test]
        fn normal() {
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(1.0, 3.0, 3.0)),
                Vec3::new(0.0, 1.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(2.0, 2.0, 3.0)),
                Vec3::new(1.0, 0.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(0.0, 2.0, 3.0)),
                Vec3::new(-1.0, 0.0, 0.0)
            );
        }
    }
}

#[cfg(test)]
mod matrix {
    use crate::matrix::*;

    #[test]
    fn arg_count() {
        assert_eq!(count_args!(), 0);
        assert_eq!(count_args!(1+1), 1);
        assert_eq!(count_args!(4, 3), 2);
    }

    #[test]
    fn expr_count() {
        assert_eq!(count_expr!(), 0);
        assert_eq!(count_expr!(1+1), 1);
        assert_eq!(count_expr!(4, 3; 2, 3, 4), 2);
        assert_eq!(count_expr!(4, 3; 2, 3; 4), 3);
    }

    #[test]
    fn new() {
        assert_eq!(
            matrix![
                0, 0, 0;
                0, 0, 0;
                0, 0, 0;
            ],
            Matrix::new(3, 3)
        );
        assert_eq!(
            matrix![
                0, 0;
                0, 0;
                0, 0;
            ],
            Matrix::new(3, 2)
        );
        assert_eq!(
            matrix![
                0, 0, 0;
                0, 0, 0;
            ],
            Matrix::new(2, 3)
        );
    }

    #[test]
    fn at() {
        assert_eq!(
            *matrix![
                0, 2, 0;
                0, 0, 4;
            ].at(2, 3),
            4
        );
        assert_eq!(
            *matrix![
                0, 2, 0;
                0, 0, 4;
            ].at(1, 2),
            2
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            (matrix![
                1, 2, 3;
                4, 5, 6;
            ] + matrix![
                1, 3, 5;
                2, 4, 6;
            ]).unwrap(),
            matrix![
                2, 5, 8;
                6, 9, 12;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn add_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] + matrix![
            1, 3, 5;
            2, 4, 6;
        ]).unwrap();
    }

    #[test]
    fn sub() {
        assert_eq!(
            (matrix![
                1, 2, 3;
                4, 5, 6;
            ] - matrix![
                1, 3, 5;
                2, 4, 6;
            ]).unwrap(),
            matrix![
                0, -1, -2;
                2, 1, 0;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn sub_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] - matrix![
            1, 3, 5;
            2, 4, 6;
        ]).unwrap();
    }

    #[test]
    fn scale() {
        assert_eq!(
            matrix![
                1, 2, 3;
                4, 5, 6;
            ] * 2,
            matrix![
                2, 4, 6;
                8, 10, 12;
            ]
        );
        assert_eq!(
            matrix![
                1, 2, 4;
                4, 8, 6;
            ] / 2,
            matrix![
                0, 1, 2;
                2, 4, 3;
            ]
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            (matrix![
                1, 2;
                3, 4;
                5, 6;
            ] * matrix![
                1, 0;
                0, 1;
            ]).unwrap(),
            matrix![
                1, 2;
                3, 4;
                5, 6;
            ]
        );
        assert_eq!(
            (matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ] * matrix![
                1, 0, 0;
                0, 1, 0;
                0, 0, 1;
            ]).unwrap(),
            matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ]
        );
        assert_eq!(
            (matrix![
                1, 2;
                3, 4;
                5, 6;
            ] * matrix![
                1, 0;
                1, 1;
            ]).unwrap(),
            matrix![
                3, 2;
                7, 4;
                11, 6;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn mul_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] * matrix![
            1, 2;
            4, 5;
            7, 8;
        ]).unwrap();
    }

    #[test]
    fn transpose() {
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
                5, 6;
            ].transpose(),
            matrix![
                1, 3, 5;
                2, 4, 6;
            ]
        );
    }

    #[test]
    fn cofactor() {
        assert_eq!(
            *matrix![
                1, 2;
                3, 4;
            ].cofactors(),
            matrix![
                1, -2;
                -3, 4;
            ]
        );
        assert_eq!(
            *matrix![
                1, 2;
                3, 4;
                5, 6;
            ].cofactors(),
            matrix![
                1, -2;
                -3, 4;
                5, -6;
            ]
        );
    }

    #[test]
    fn minor() {
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
            ].minor(1, 2),
            matrix![
                3;
            ]
        );
        assert_eq!(
            matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ].minor(1, 2),
            matrix![
                3, 8;
                5, 9;
            ]
        );
    }

    #[test]
    fn det() {
        assert_eq!(
            matrix![
                1, 0;
                0, 1;
            ].det().unwrap(),
            1
        );
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
            ].det().unwrap(),
            -2
        );
        assert_eq!(
            matrix![
                1, 2, 3;
                3, 2, 1;
                2, 1, 3;
            ].det().unwrap(),
            -6
        );
    }
}
