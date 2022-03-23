

    pub fn test(input : &str) -> bool {
        // fail fast if empty
        if input.is_empty() {
            return false;
        }

        // loop through the chars of the input string, extracting only digits into an array
        let cc_nums : Vec<u32> = input.chars()
                                .filter(|c| c.is_digit(10))
                                .map(|c| c.to_digit(10).unwrap()).collect();

        // we need a minimum of 2 numbers to calculate luhn
        if cc_nums.len() < 2 {
            return false;
        }

        /*  
            define known math for the luhn algorithm via array : if doubling a number results in a two digit number,
            add the digits of the product to get a single digit number. This is simple pre math, for a lookup style 
            solution, vs inline math. for x being any number 0 through 9, x >= 5 then (x * 2) - 9 else x * 2
        */
        let known_luhn_values : [u32;10] = [0,2,4,6,8,1,3,5,7,9];

        // this is to be the summed result of the luhn math pre mod test
        let mut completed_sum : u32 = 0;

        // loop through the inputs in reverse as per the luhn spec
        for (index,cc_num) in cc_nums.iter().rev().enumerate()  {
        completed_sum += if index % 2 == 1 { known_luhn_values[*cc_num as usize] } else { *cc_num }
        }

        // the completedSum should be divisible by 10
        return completed_sum % 10 == 0;
    }

