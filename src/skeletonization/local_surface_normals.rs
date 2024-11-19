use crate::common::{Point, V};
use std::process::Command;

pub fn local_surface_normals_per_point(points: &Vec<Point>) -> Vec<V> {
    let input_filename = save_to_pcd(points);
    let output_filename = call_cloud_compare(&input_filename);
    read_normals(&output_filename)
}

fn save_to_pcd(points: &Vec<Point>) -> String {
    use pcd_rs::{DynRecord, DynWriter, Field, ValueKind};

    const FILENAME: &str = "./CloudCompare/input_pointcloud.pcd";

    let schema = vec![
        ("x", ValueKind::F32, 1),
        ("y", ValueKind::F32, 1),
        ("z", ValueKind::F32, 1),
    ];

    let mut writer: DynWriter<_> = pcd_rs::WriterInit {
        width: 169,
        height: 1,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: Some(pcd_rs::Schema::from_iter(schema)),
    }
    .create(FILENAME)
    .expect("Writer to get created");

    for point in points {
        writer
            .push(&DynRecord(vec![
                Field::F32(vec![point.x]),
                Field::F32(vec![point.y]),
                Field::F32(vec![point.z]),
            ]))
            .expect("Successful push");
    }

    writer.finish().expect("Writer to finish");
    return FILENAME.to_owned();
}

// TODO it's dangerous to call another file, once the command is stable, hardcode it
fn call_cloud_compare(_input_filename: &str) -> String {
    const RESULT_FILENAME: &str = "./CloudCompare/output_pointcloud.pcd";

    let output = Command::new("./CloudCompare/cloud_compare_normals.bat")
        .output()
        .expect("cloudcompare execution");

    if !output.status.success() {
        println!("CloudCompare status: {}", output.status);
        println!(
            "CloudCompare stdout: {}",
            String::from_utf8(output.stdout).expect("ASCII output")
        );
        println!(
            "CloudCompare stderr: {}",
            String::from_utf8(output.stderr).expect("ASCII output")
        );
        panic!("CloudCompare call failed");
    }

    return RESULT_FILENAME.to_owned();
}

fn read_normals(output_filename: &str) -> Vec<V> {
    let reader = pcd_rs::DynReader::open(output_filename).expect("cloudcompare result to exist");
    println!("{:?}", reader.meta().field_defs);

    reader
        .map(|record| {
            let record: pcd_rs::DynRecord = record.unwrap();
            let fields = &record.0;
            let normal_x: f32 = fields[0].to_value().unwrap();
            let normal_y: f32 = fields[1].to_value().unwrap();
            let normal_z: f32 = fields[2].to_value().unwrap();
            V::new(normal_x, normal_y, normal_z)
        })
        .collect()
}
