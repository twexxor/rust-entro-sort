pub fn entro_sort_ascending<T: Ord>(input: &mut [T]) {
    let mut step_count: usize;
    let mut i: usize;
    let mut j: usize;
    let mut is_sorted: bool;

    if input.len() > 1 {
        i = 0;
        j = (input.len() >> 12) + (input.len() >> 7) + 1;
        is_sorted = false;

        while is_sorted == false {
            step_count = (input.len() >> 15) + (input.len() >> 14) + 1;
            is_sorted = true;

            while i < j {
                while j != input.len() {
                    if input[i] > input[j] {
                        input.swap(i, j);
                    }

                    i += 1;
                    j += 1;
                }

                if (input.len() - i) >> 12 != 0 {
                    step_count += 1;
                }

                i += step_count;

                if i >= j {
                    i = j - 1;
                }

                while i != 0 {
                    i -= 1;
                    j -= 1;

                    if input[i] > input[j] {
                        input.swap(i, j);
                        is_sorted = false;
                    }
                }

                j -= 1;
            }

            if is_sorted == false {
                i = 0;
                j = (input.len() >> 15) + (input.len() >> 14) + 8;
            }
        }
    }
}

pub fn entro_sort_descending<T: Ord>(input: &mut [T]) {
    let mut step_count: usize;
    let mut i: usize;
    let mut j: usize;
    let mut is_sorted: bool;

    if input.len() > 1 {
        i = 0;
        j = (input.len() >> 12) + (input.len() >> 7) + 1;
        is_sorted = false;

        while is_sorted == false {
            step_count = (input.len() >> 15) + (input.len() >> 14) + 1;
            is_sorted = true;

            while i < j {
                while j != input.len() {
                    if input[i] < input[j] {
                        input.swap(i, j);
                    }

                    i += 1;
                    j += 1;
                }

                if (input.len() - i) >> 12 != 0 {
                    step_count += 1;
                }

                i += step_count;

                if i >= j {
                    i = j - 1;
                }

                while i != 0 {
                    i -= 1;
                    j -= 1;

                    if input[i] < input[j] {
                        input.swap(i, j);
                        is_sorted = false;
                    }
                }

                j -= 1;
            }

            if is_sorted == false {
                i = 0;
                j = (input.len() >> 15) + (input.len() >> 14) + 8;
            }
        }
    }
}
