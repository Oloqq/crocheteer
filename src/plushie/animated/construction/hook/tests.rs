use pretty_assertions::assert_eq as q;

use super::*;

const COLOR: colors::Color = colors::RED;

impl Hook {
    pub fn test_perform(self, action: &Action) -> Result<Self, HookError> {
        self.perform(action, &HookParams::default())
    }
}

#[test]
fn test_start_with_magic_ring() {
    let h = Hook::start_with(&MR(3), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.now.cursor, 4);
    q!(
        h.edges,
        Edges::from_unchecked(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
    );
    q!(h.part_limits, vec![0]);
}

#[test]
fn test_part_limits_gets_filled() {
    let h = Hook::start_with(&MR(3), COLOR).unwrap();
    q!(h.part_limits, vec![0]);
    let h = h.test_perform(&MRConfigurable(6, "main".into())).unwrap();
    q!(h.part_limits, vec![0, 4]);
    let result = h.finish();
    q!(result.part_limits, vec![0, 4, 11]);
}

#[test]
fn test_start_with_magic_ring_configurable() {
    let h = Hook::start_with(&MRConfigurable(3, "main".into()), COLOR).unwrap();
    q!(h.peculiar.len(), 1);
    assert!(matches!(h.peculiar.get(&0).unwrap(), Peculiarity::Locked));

    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.now.cursor, 4);
    q!(
        h.edges,
        Edges::from_unchecked(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
    );
}

#[test]
#[ignore = "chain starter is disabled"]
fn test_start_with_chain() {
    let h = Hook::start_with(&Ch(3), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([0, 1, 2]));
    q!(h.now.cursor, 3);
    q!(h.edges, Edges::from(vec![vec![1], vec![2], vec![], vec![]]));
    q!(
        h.edges,
        Edges::from_unchecked(vec![vec![], vec![0], vec![1], vec![]])
    );
}

#[test]
fn test_test_perform_sc() {
    let mut h = Hook::start_with(&MR(6), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([1, 2, 3, 4, 5, 6]));
    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from([2, 3, 4, 5, 6, 7]));
    q!(h.now.cursor, 8);

    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from([3, 4, 5, 6, 7, 8]));
    q!(h.now.cursor, 9);
}

#[test]
fn test_test_perform_inc() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    h = h.test_perform(&Inc).unwrap();
    q!(h.now.anchors, Queue::from([2, 3, 4, 5]));
    q!(h.now.cursor, 6);
    q!(
        h.edges,
        Edges::from_unchecked(vec![
            vec![],
            vec![0],
            vec![0, 1],
            vec![0, 2],
            vec![3, 1],
            vec![4, 1],
            vec![]
        ])
    )
}

#[test]
fn test_test_perform_dec() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    h = h.test_perform(&Dec).unwrap();
    q!(h.now.anchors, Queue::from([3, 4]));
    q!(h.now.cursor, 5);
}

#[test]
fn test_test_perform_fo_after_full_round() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.now.cursor, 4);
    q!(h.edges.len(), 5);
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from([4, 5, 6]));
    q!(h.now.cursor, 7);
    q!(h.edges.len(), 8);
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5],       // 4
            vec![6],       // 5
            vec![],        //6
            vec![]
        ])
    );
    h = h
        .perform(
            &FO,
            &HookParams {
                tip_from_fo: true,
                enforce_counts: false,
            },
        )
        .unwrap();
    q!(h.now.anchors, Queue::from([]));
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5, 7],    // 4
            vec![6, 7],    // 5
            vec![7],       // 6
            vec![],        // 7
            vec![]
        ])
    );
}

#[test]
fn test_error_on_stitch_after_fo() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    h = h
        .perform(
            &FO,
            &HookParams {
                tip_from_fo: true,
                enforce_counts: false,
            },
        )
        .unwrap();
    h.clone()
        .test_perform(&Sc)
        .expect_err("Can't continue after FO");
    h.clone()
        .test_perform(&Inc)
        .expect_err("Can't continue after FO");
    h.clone()
        .test_perform(&Dec)
        .expect_err("Can't continue after FO");
}

#[test]
fn test_goto_after_fo() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    h = h.test_perform(&Mark(0)).unwrap();
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from([4, 5, 6]));
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5],       // 4
            vec![6],       // 5
            vec![],        // 6
            vec![]
        ])
    );
    h = h
        .perform(
            &FO,
            &HookParams {
                tip_from_fo: true,
                enforce_counts: false,
            },
        )
        .unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3], // 0
            vec![2, 4],    // 1
            vec![3, 5],    // 2
            vec![4, 6],    // 3
            vec![5, 7],    // 4
            vec![6, 7],    // 5
            vec![7],       // 6
            vec![],        // 7
            vec![]
        ])
    );
    q!(h.now.anchors, Queue::from([]));
    h = h.test_perform(&Goto(0)).unwrap();
    q!(h.now.cursor, 8);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.override_previous_stitch, Some(3));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3],     // 0 - root
            vec![2, 4, 8],     // 1 - ring
            vec![3, 5, 9],     // 2 - ring
            vec![4, 6, 8, 10], // 3 - ring
            vec![5, 7],        // 4 - sc
            vec![6, 7],        // 5 - sc
            vec![7],           // 6 - sc
            vec![],            // 7 - tip 1
            vec![9],           // 8 - sc
            vec![10],          // 9 - sc
            vec![],            // 10 - sc
            vec![],
        ])
    );
}

#[test]
fn test_chain_simple() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    h = h.test_perform(&Ch(3)).unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![],
            vec![],
        ])
    );
}

#[test]
fn test_attach1() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    let attach_here = 0;
    let return_here = 1;
    h = h.test_perform(&Mark(attach_here)).unwrap();
    q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Mark(return_here)).unwrap();
    q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
    q!(
        h.edges,
        Edges::from(vec![
            vec![],     // 0: root
            vec![0],    // 1: mr 1
            vec![0, 1], // 2: mr 2
            vec![0, 2], // 3: mr 3, mark
            vec![1, 3], // 4: sc
            vec![],
        ])
    );
    h = h.test_perform(&Attach(attach_here, 3)).unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![],     // 0: root
            vec![0],    // 1: mr 1
            vec![0, 1], // 2: mr 2
            vec![0, 2], // 3: mr 3, mark
            vec![1, 3], // 4: sc 1
            vec![4],    // 5: ch 1
            vec![5],    // 6: ch 2
            vec![6],    // 7: ch 3
            vec![3, 7], // 8: attaching
            vec![],
        ])
    );
    let part_a = h.now;
    let part_b = h.labels.get(&return_here).unwrap();

    q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));

    q!(part_b.anchors, Queue::from(vec![2, 8, 7, 6, 5]));
}

#[test]
fn test_sc_after_attach() {
    let mut h = Hook::start_with(&MR(3), COLOR).unwrap();
    let attach_here = 0;
    let return_here = 1;
    h = h.test_perform(&Mark(attach_here)).unwrap();
    q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Mark(return_here)).unwrap();
    q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
    q!(
        h.edges,
        Edges::from(vec![
            vec![],     // 0: root
            vec![0],    // 1: mr 1
            vec![0, 1], // 2: mr 2
            vec![0, 2], // 3: mr 3, mark
            vec![1, 3], // 4: sc
            vec![],
        ])
    );
    h = h.test_perform(&Attach(attach_here, 3)).unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![],     // 0: root
            vec![0],    // 1: mr 1
            vec![0, 1], // 2: mr 2
            vec![0, 2], // 3: mr 3, mark
            vec![1, 3], // 4: sc 1
            vec![4],    // 5: ch 1
            vec![5],    // 6: ch 2
            vec![6],    // 7: ch 3
            vec![3, 7], // 8
            vec![],
        ])
    );
    {
        let part_a = &h.now;

        q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));
    }

    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from(vec![5, 6, 7, 9]));

    q!(
        h.edges,
        Edges::from(vec![
            vec![],     // 0: root
            vec![0],    // 1: mr 1
            vec![0, 1], // 2: mr 2
            vec![0, 2], // 3: mr 3, mark
            vec![1, 3], // 4: sc 1
            vec![4],    // 5: ch 1
            vec![5],    // 6: ch 2
            vec![6],    // 7: ch 3
            vec![3, 7], // 8: attaching
            vec![4, 8], // 9: sc
            vec![],
        ])
    );

    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from(vec![6, 7, 9, 10]));
    h = h.test_perform(&Sc).unwrap();
    q!(h.now.anchors, Queue::from(vec![7, 9, 10, 11]));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Sc).unwrap();

    h.finish();
}

#[test]
fn test_split_moment() {
    let mut source = Moment {
        cursor: 20,
        anchors: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].into(),
        working_on: WorkingLoops::Both,
        limb_ownerhip: 0,
    };
    let (moment_a, moment_b) = split_moment(&mut source, 6, [13, 14, 15, 16].into());
    println!("{:?} {:?}", moment_a.anchors, moment_b.anchors);
    q!(moment_a.anchors.len(), 9);
    q!(moment_b.anchors.len(), 9);
}

#[test]
fn test_starting_from_color() {
    let mut flow = crate::acl::simple_flow::SimpleFlow::new(vec![Color(colors::RED), MR(3), Ch(3)]);
    let mut h = Hook::from_starting_sequence(&mut flow).unwrap();
    h = h.test_perform(&flow.next().unwrap()).unwrap();
    q!(
        h.edges,
        Edges::from(vec![
            vec![1, 2, 3],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![],
            vec![],
        ])
    );
}

// #[test]
// fn test_multipart_start() {
//     let mut h = Hook::start_with(&MR(3)).unwrap();
//     let attach_here = 0;
//     let return_here = 1;
//     h = h.perform(&Mark(attach_here)).unwrap();
//     q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
//     h = h.perform(&Sc).unwrap();
//     h = h.perform(&Mark(return_here)).unwrap();
//     q!(h.now.anchors, Queue::from(vec![2, 3, 4]));
//     q!(h.now.round_left, 2);
//     q!(
//         h.edges,
//         Edges::from(vec![
//             vec![],     // 0: root
//             vec![0],    // 1: mr 1
//             vec![0, 1], // 2: mr 2
//             vec![0, 2], // 3: mr 3, mark
//             vec![1, 3], // 4: sc
//             vec![],
//         ])
//     );
//     h = h.perform(&Attach(attach_here, 3)).unwrap();
//     q!(
//         h.edges,
//         Edges::from(vec![
//             vec![],     // 0: root
//             vec![0],    // 1: mr 1
//             vec![0, 1], // 2: mr 2
//             vec![0, 2], // 3: mr 3, mark
//             vec![1, 3], // 4: sc 1
//             vec![4],    // 5: ch 1
//             vec![5],    // 6: ch 2
//             vec![6],    // 7: ch 3
//             vec![3, 7], // 8
//             vec![],
//         ])
//     );
//     {
//         let part_a = &h.now;

//         q!(part_a.anchors, Queue::from(vec![4, 5, 6, 7]));
//         q!(part_a.round_left, 4);
//     }

//     h = h.perform(&Sc).unwrap();
//     q!(h.now.anchors, Queue::from(vec![5, 6, 7, 9]));
//     q!(h.now.round_left, 3);

//     q!(
//         h.edges,
//         Edges::from(vec![
//             vec![],     // 0: root
//             vec![0],    // 1: mr 1
//             vec![0, 1], // 2: mr 2
//             vec![0, 2], // 3: mr 3, mark
//             vec![1, 3], // 4: sc 1
//             vec![4],    // 5: ch 1
//             vec![5],    // 6: ch 2
//             vec![6],    // 7: ch 3
//             vec![3, 7], // 8: attaching
//             vec![4, 8], // 9: sc
//             vec![],
//         ])
//     );

//     h = h.perform(&Sc).unwrap();
//     q!(h.now.anchors, Queue::from(vec![6, 7, 9, 10]));
//     h = h.perform(&Sc).unwrap();
//     q!(h.now.anchors, Queue::from(vec![7, 9, 10, 11]));
//     h = h.perform(&Sc).unwrap();
//     h = h.perform(&Sc).unwrap();

//     let result = h.finish();
//     q!(result.nodes.len(), result.colors.len());
// }
