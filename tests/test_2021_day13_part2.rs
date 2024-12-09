mod common;

#[test]
#[rustfmt::skip]
fn run_test() {
    let result = common::run_challenge("2021_day13_part2");
    let result_lines: Vec<&str> = result.trim().lines().collect();
    assert_eq!(result_lines.len(), 7);
    assert_eq!(result_lines[0].trim_end(), "The code displayed on the grid is:");
    assert_eq!(result_lines[1].trim_end(), "###  #  #  ##  #    ###   ##  ###   ##");
    assert_eq!(result_lines[2].trim_end(), "#  # #  # #  # #    #  # #  # #  # #  #");
    assert_eq!(result_lines[3].trim_end(), "#  # #### #  # #    #  # #    #  # #  #");
    assert_eq!(result_lines[4].trim_end(), "###  #  # #### #    ###  #    ###  ####");
    assert_eq!(result_lines[5].trim_end(), "# #  #  # #  # #    # #  #  # # #  #  #");
    assert_eq!(result_lines[6].trim_end(), "#  # #  # #  # #### #  #  ##  #  # #  #");
}
