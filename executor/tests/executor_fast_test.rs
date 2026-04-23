use executor::Executor;
use executor::Pose;

mod fast_tests{
    use super::*;
    
    //F for M, L, R
    #[test]
    fn should_return_x_plus_2_given_status_is_fast_command_is_fm_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("FM");

        // then
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_facing_is_n_given_status_is_fast_command_is_fl_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("FL");

        // then
        let expected_pose = Pose::new(1, 0, 'n');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_facing_is_s_given_status_is_fast_command_is_fr_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("FR");

        // then
        let expected_pose = Pose::new(1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    //BF for M, L, R
    #[test]
    fn should_return_x_minus_2_given_status_is_fast_command_is_bfm_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BFM");

        // then
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_facing_is_s_given_status_is_fast_command_is_bfl_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BFL");

        // then
        let expected_pose = Pose::new(-1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_facing_is_n_given_status_is_reverse_and_fast_command_is_bfr_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BFR");

        // then
        let expected_pose = Pose::new(-1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    //FF for M, L, R
    #[test]
    fn should_return_y_plus_1_given_status_is_fast_command_is_ffm_and_facing_is_n(){
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("FFM");

        // then
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
    
}