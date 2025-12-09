macro_rules! ternary {
    ($cond: expr, $expr_a: expr, $expr_b: expr) => {
        if $cond { $expr_a } else { $expr_b }
    };
}

macro_rules! const_while {
    ($i: ident, $start: expr, $max: expr, $block: block) => {
        let mut $i = $start;

        while $i < $max {
            $block;
            $i += 1;
        }
    };
    ($i: ident, $max: expr, $block: block) => {
        const_while!($i, 0, $max, $block)
    };
}

macro_rules! bench {
    ($block: block) => {
        use colored::Colorize;

        let instant = std::time::Instant::now();
        $block;
        let elapsed = instant.elapsed().as_millis();
        println!("Ran in {} ms.\n", elapsed.to_string().green());
    };
}

pub(crate) use bench;
pub(crate) use const_while;
pub(crate) use ternary;
