use executor::Executor;
use executor::Pose;

mod reverse_tests{
    use super::*;

    #[test]
    fn should_return_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BM");

        // then
        let expected_pose = Pose::new(-1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_s_given_status_is_reverse_command_is_l_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BL");

        // then
        let expected_pose = Pose::new(0,0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_n_given_status_is_reverse_command_is_r_and_facing_is_e(){
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BR");

        // then
        let expected_pose = Pose::new(0,0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_bbm_and_facing_is_n(){
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("BBM");

        // then
        let expected_pose = Pose::new(0,1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    

}

