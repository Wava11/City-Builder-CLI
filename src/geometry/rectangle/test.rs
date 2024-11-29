use ratatui::layout::Rect;
#[cfg(test)]
mod subtract {

    #[cfg(test)]
    mod disjoint_rectangles {
        use crate::geometry::rectangle::Rectangle;

        #[test]
        fn returns_only_self() {
            let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
            let rect2 = Rectangle::from_coordinates((15, 15), (20, 20));

            let result = rect1.subtract(&rect2);
            assert_eq!(result, vec![rect1]);
        }
    }

    #[cfg(test)]
    mod identical_rectangles {
        use crate::geometry::rectangle::Rectangle;

        #[test]
        fn returns_empty_vec() {
            let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
            let rect2 = Rectangle::from_coordinates((5, 5), (10, 10));

            let result = rect1.subtract(&rect2);
            assert_eq!(result, vec![]);
        }
    }

    #[cfg(test)]
    mod intersecting_rectangles {
        mod same_width_self_higher_than_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_top_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((5, 8), (10, 13));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 5), (10, 7))]);
            }
        }

        mod same_width_self_lower_than_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_top_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((5, 2), (10, 8));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 9), (10, 10))]);
            }
        }

        mod same_width_same_top_y_self_ends_below_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_bottom_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((5, 5), (10, 8));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 9), (10, 10))]);
            }
        }

        mod same_width_same_top_y_which_is_0_self_ends_below_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_bottom_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 0), (10, 10));
                let rect2 = Rectangle::from_coordinates((5, 0), (10, 8));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 9), (10, 10))]);
            }
        }

        mod same_width_same_bottom_y_which_is_max_self_starts_above_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_top_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 10), (10, usize::MAX));
                let rect2 = Rectangle::from_coordinates((5, 15), (10, usize::MAX));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 10), (10, 14))]);
            }
        }

        mod same_height_self_ends_farther_right_than_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_right_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((3, 5), (8, 10));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((9, 5), (10, 10))]);
            }
        }

        mod same_bottom_right_self_top_left_is_higher_and_further_left {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_top_strip_of_self_and_the_left_strip_which_is_under_the_top_strip() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((7, 7), (10, 10));

                let result = rect1.subtract(&rect2);
                assert_eq!(
                    result,
                    vec![
                        Rectangle::from_coordinates((5, 5), (10, 6)),
                        Rectangle::from_coordinates((5, 7), (6, 10)),
                    ]
                );
            }
        }

        mod self_strictly_contains_other {
            use std::collections;

            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_four_rectangles_outside_of_other_but_inside_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((7, 7), (9, 9));

                let result = rect1.subtract(&rect2);
                assert_eq!(
                    collections::BTreeSet::from_iter(result.iter()),
                    collections::BTreeSet::from_iter(vec![
                        Rectangle::from_coordinates((5, 5), (10, 6)),
                        Rectangle::from_coordinates((5, 7), (6, 9)),
                        Rectangle::from_coordinates((10, 7), (10, 9)),
                        Rectangle::from_coordinates((5, 10), (10, 10)),
                    ].iter())
                );
            }
        }
    }
}
