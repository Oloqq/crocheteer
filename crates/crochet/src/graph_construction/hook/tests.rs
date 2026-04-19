use super::{Edges, Hook, HookParams, Queue};

use pretty_assertions::assert_eq as q;

use crate::{
    ColorRgb,
    acl::{Action, Flow, Label, SimpleFlow},
    data::Peculiarity,
    graph_construction::ErrorCode,
};
use Action::*;

const COLOR: ColorRgb = [255, 0, 0];

impl Hook {
    pub fn test_perform(self, action: &Action) -> Result<Self, ErrorCode> {
        self.perform(action, None)
    }
}

fn start_mr(mr_count: usize) -> Hook {
    let mut h = Hook::new(HookParams::default());
    h = h.perform(&Action::Color(COLOR), None).unwrap();
    h = h.perform(&BeginPart, None).unwrap();
    h = h.perform(&MR(mr_count), None).unwrap();
    h
}

#[test]
fn test_start_with_magic_ring() {
    let h = start_mr(3);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.now.cursor, 4);
    q!(
        h.edges,
        Edges::from(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
    );
}

#[test]
fn test_end_part_resets_anchors() {
    let mut h = start_mr(3);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.now.cursor, 4);
    q!(
        h.edges,
        Edges::from(vec![vec![], vec![0], vec![0, 1], vec![0, 2], vec![]])
    );
    h = h.perform(&EndPart, None).unwrap();
    q!(h.part_limits, vec![4]);
    q!(h.now.anchors, Queue::new());
    q!(h.now.cursor, 4);
}

#[test]
fn test_test_perform_sc() {
    let mut h = start_mr(6);
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
    let mut h = start_mr(3);
    h = h.test_perform(&Inc).unwrap();
    q!(h.now.anchors, Queue::from([2, 3, 4, 5]));
    q!(h.now.cursor, 6);
    q!(
        h.edges,
        Edges::from(vec![
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
    let mut h = start_mr(3);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    h = h.test_perform(&Dec).unwrap();
    q!(h.now.anchors, Queue::from([3, 4]));
    q!(h.now.cursor, 5);
}

#[test]
fn test_test_perform_fo_after_full_round() {
    let mut h = start_mr(3);
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
    h.params.tip_from_fo = true;
    h = h.test_perform(&FO).unwrap();
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
    let mut h = start_mr(3);
    h.params.tip_from_fo = true;
    h = h.test_perform(&FO).unwrap();
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
    let mut h = start_mr(3);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    h = h.test_perform(&Mark("0".into())).unwrap();
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
    h.params.tip_from_fo = true;
    h = h.test_perform(&FO).unwrap();
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
    h = h.test_perform(&Goto("0".into())).unwrap();
    q!(h.now.cursor, 8);
    q!(h.now.anchors, Queue::from([1, 2, 3]));
    q!(h.override_previous_node, Some(3));
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
fn test_attach1() {
    let mut h = start_mr(3);
    let attach_here: Label = "0".into();
    let return_here: Label = "1".into();
    h = h.test_perform(&Mark(attach_here.clone())).unwrap();
    q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Mark(return_here.clone())).unwrap();
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
    let mut h = start_mr(3);
    let attach_here: Label = "0".into();
    let return_here: Label = "1".into();
    h = h.test_perform(&Mark(attach_here.clone())).unwrap();
    q!(h.now.anchors, Queue::from(vec![1, 2, 3]));
    h = h.test_perform(&Sc).unwrap();
    h = h.test_perform(&Mark(return_here.clone())).unwrap();
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
fn test_starting_from_color() {
    let mut flow = SimpleFlow::new(vec![Color(COLOR), MR(3), Sc, Sc, Sc]);
    let mut h = Hook::new(HookParams::default());
    while let Some(a) = flow.next() {
        h = h.perform(&a, None).unwrap();
    }
    q!(
        &h.edges.data()[0..4],
        &[vec![], vec![0], vec![0, 1], vec![0, 2]]
    );
}

#[test]
fn test_mark_to_node() {
    let mut h = start_mr(3);
    let mark0: Label = "0".into();
    h = h.test_perform(&Mark(mark0.clone())).unwrap();
    q!(*h.mark_to_node.get("0").unwrap(), 3);
    let graph = h.finish();
    q!(*graph.mark_to_node.get("0").unwrap(), 3);
}

#[test]
fn test_next_anchor_in_dec_does_not_panic() {
    let mut h = start_mr(3);
    q!(h.now.anchors.len(), 3);
    h = h.test_perform(&Dec).unwrap();
    q!(h.now.anchors.len(), 2);
    h = h.test_perform(&Dec).unwrap();
    q!(h.now.anchors.len(), 1);
    assert!(matches!(
        h.test_perform(&Dec).unwrap_err(),
        ErrorCode::NoAnchorToPullThrough
    ));
}

#[test]
fn test_colors_are_registered() {
    let mut h = start_mr(3);
    q!(h.nodes.len(), 4); // 3 + virtual MR root
    q!(h.nodes[0].color, COLOR);
    q!(h.nodes[1].color, COLOR);
    q!(h.nodes[2].color, COLOR);
    q!(h.nodes[3].color, COLOR);
    h = h.test_perform(&Color([255, 177, 255])).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(h.nodes[4].color, [255, 177, 255]);
}

#[test]
fn test_mr_root_peculiarity_is_registered() {
    let h = start_mr(3);
    q!(h.nodes[0].peculiarity, Some(Peculiarity::Locked));
}

#[test]
fn test_parents_are_registered() {
    let mut h = start_mr(3);
    h = h.test_perform(&FLO).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(h.nodes[0].parent, None);
    q!(h.nodes[1].parent, Some(0));
    q!(h.nodes[2].parent, Some(0));
    q!(h.nodes[3].parent, Some(0));
    q!(h.nodes[4].parent, Some(1));
}

#[test]
fn test_flo_is_registered() {
    let mut h = start_mr(3);
    h = h.test_perform(&FLO).unwrap();
    h = h.test_perform(&Sc).unwrap();
    q!(h.nodes[4].peculiarity, Some(Peculiarity::FLO((2, 1, 0))));
}

// slst all around is possible in reality, so should be possible here
// slst in reality is difficult to pull through,
// is there a legitimate reason one would want to pull through a slst?
//
// #[test]
// fn test_slst_all_around() {
//     let mut h = start_mr(3);
//     h = h.test_perform(&Sc).unwrap();
//     h = h.test_perform(&Sc).unwrap();
//     h = h.test_perform(&Sc).unwrap();
//     q!(h.now.anchors, Queue::from([4, 5, 6]));
//     q!(h.now.cursor, 7);
//     // q!(
//     //     h.edges,
//     //     Edges::from(vec![
//     //         vec![],
//     //         vec![0],
//     //         vec![0, 1],
//     //         vec![0, 2],
//     //         vec![1, 3],
//     //         vec![2, 4],
//     //         vec![3, 5],
//     //         vec![]
//     //     ])
//     // );
//     // h = h.test_perform(&Slst).unwrap();
//     // q!(h.now.cursor, 7);
//     // q!(
//     //     h.edges,
//     //     Edges::from(vec![
//     //         vec![],
//     //         vec![0],
//     //         vec![0, 1],
//     //         vec![0, 2],
//     //         vec![1, 3],
//     //         vec![2, 4],
//     //         vec![3, 5],
//     //         vec![]
//     //     ])
//     // );
// }
