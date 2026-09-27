use std::error::Error;
use tars::*;
mod experiments;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut train = experiments::choose();

    let optimizer = BGD::new(train.lr);
    let mut prev_cost = cost(&train.model, &train.dataset);
    println!("epoch: 000000, cost is:{:014.8}", prev_cost);

    for i in 1..=train.epochs {
        let grad = num_grad(&train.model, &train.dataset);
        optimizer.step(&mut train.model, &grad);
        let curr_cost = cost(&train.model, &train.dataset);
        if i % (train.epochs / 20) == 0 {
            println!(
                "epoch: {:06.0}, cost is:{:014.8}, {:07.3}% better",
                i,
                curr_cost,
                ((prev_cost - curr_cost) * 100.0) / prev_cost
            );
            prev_cost = curr_cost;
        }
    }

    for d in train.dataset {
        println!(
            "Para as entradas: {:?} o modelo retorna: {:?} esperado:{:?} ",
            d.input,
            train.model.forward(&d.input),
            d.target
        );
    }
    let mut file = File::create("npu/weights.mem")?;
    for module in &train.model.modules {
        match module {
            AnyModule::Linear(linear) => {
                for neuron in &linear.weights {
                    for weight in neuron {
                        writeln!(file, "{:08X}", weight.to_bits())?;
                    }
                }
            }
            AnyModule::Activation(_) => {}
        }
    }
    Ok(())
}
