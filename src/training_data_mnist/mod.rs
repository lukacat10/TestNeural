use mnist::*;

struct TrainingData {
    trn_img: Box<Vec<u8>>,
    trn_lbl: Box<Vec<u8>>,

    val_img: Box<Vec<u8>>,
    val_lbl: Box<Vec<u8>>,
    
    tst_img: Box<Vec<u8>>,
    tst_lbl: Box<Vec<u8>>,
}

fn get_training_data() -> TrainingData {

    let Mnist {
        trn_img,
        trn_lbl,
        
        val_img,
        val_lbl,

        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(50_000)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();

    let training_data = TrainingData{
        trn_img: Box::new(trn_img),
        trn_lbl: Box::new(trn_lbl),
        val_img: Box::new(val_img),
        val_lbl: Box::new(val_lbl),
        tst_img: Box::new(tst_img),
        tst_lbl: Box::new(tst_lbl),
    };

    // let image_num = 0;
    // // Can use an Array2 or Array3 here (Array3 for visualization)
    // let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img)
    //     .expect("Error converting images to Array3 struct")
    //     .map(|x| *x as f32 / 256.0);
    // println!("{:#.1?}\n", train_data.slice(s![image_num, .., ..]));
    // // Convert the returned Mnist struct to Array2 format
    // let train_labels: Array2<f32> = Array2::from_shape_vec((50_000, 1), trn_lbl)
    //     .expect("Error converting training labels to Array2 struct")
    //     .map(|x| *x as f32);
    // println!(
    //     "The first digit is a {:?}",
    //     train_labels.slice(s![image_num, ..])
    // );
    // let _test_data = Array3::from_shape_vec((10_000, 28, 28), tst_img)
    //     .expect("Error converting images to Array3 struct")
    //     .map(|x| *x as f32 / 256.);
    // let _test_labels: Array2<f32> = Array2::from_shape_vec((10_000, 1), tst_lbl)
    //     .expect("Error converting testing labels to Array2 struct")
    //     .map(|x| *x as f32);

    //TODO Use the processing done here!!!!

    training_data
}
