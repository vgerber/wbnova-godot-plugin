use nova_api::paths::stream_motion_group_state::{
    stream_motion_group_state, StreamMotionGroupStatePathParameters,
    StreamMotionGroupStateQueryParameters,
};

#[tokio::test]
async fn stream_robot() {
    // Client meta
    let host = "wss://jfobbyis.instance.wandelbots.io/api/v1";
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImpGWkFUaWc5bk1TUERrT0lpV2hJSyJ9.eyJpc3MiOiJodHRwczovL2F1dGgucG9ydGFsLndhbmRlbGJvdHMuaW8vIiwic3ViIjoiZ2l0aHVifDEyMDE4NjY3IiwiYXVkIjpbIm5vdmEtYXBpIiwiaHR0cHM6Ly93YW5kZWxib3RzLmV1LmF1dGgwLmNvbS91c2VyaW5mbyJdLCJpYXQiOjE3NDE3MTM5MzgsImV4cCI6MTc0MTgwMDMzOCwic2NvcGUiOiJvcGVuaWQgcHJvZmlsZSBlbWFpbCIsImF6cCI6Im5xQnMyV0Z3RlRocjc1WHZ2eVlJa2d2b0s2MjNVdWRBIn0.birny40y5sKmuANCK-ssViy7ly6G-M1dSughmibA47J-fmcdPx6ofl90-jjSwh1aLhBpz5lD0edmBj5DdkhYQo0jTP2AwL7fgNabWL929GFWtlJLd2204IVVgOCuxEapU7YmjjQzsbBbkzM9BsGNNSlWLkg7I7-vhBoV3nii-6xMn4uWRiyf9W0K286wBeAisHRNIfZMjisEo0aAprwsqhtY_FkRdxylrGnqirIg3kkqbN3CK00pZ3UFmhUdARhu2mN7MQxzYPBovOtxnJvBGbEhYYjobG7bUkDugIi13sP2mua3-wxNfTYhoOkpBPLd7YNxDRMgHG5ZJxONLGTE2A";
    let headers = vec![("Authorization".to_owned(), format!("Bearer {token}"))];

    // request params
    let robot_data = StreamMotionGroupStatePathParameters {
        cell: "cell".to_owned(),
        motion_group: "0@ur5e".to_owned(),
    };
    let robot_query_data = StreamMotionGroupStateQueryParameters {
        response_rate: Some(100),
        response_coordinate_system: None,
        tcp: None,
    };

    println!("Connect");
    let mut socket = match stream_motion_group_state(
        host,
        &robot_data,
        &robot_query_data,
        Some(headers),
    )
    .await
    {
        Ok(socket) => socket,
        Err(err) => {
            panic!("{}", err);
        }
    };

    println!("Read");
    let mut counter = 10;
    while counter > 0 {
        let state = match socket.read() {
            Ok(message) => message,
            Err(err) => {
                panic!("{}", err);
            }
        };
        println!("{:?}", state.state.joint_position);
        counter -= 1;
    }
}
