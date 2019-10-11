use crate::clients;
use crate::ipparser;

#[test]
fn client_new() {
    let ipv4_addr: u32 = 3232235826;
    let port: u16 = 8000;
    let username = "jorge_alarcon";
    let get_only_by_mac = false;
    let drop_votes: u8 = 12;
    if let Some(_client) = clients::Client::new(ipv4_addr, port, username, get_only_by_mac, drop_votes) {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn clients_map_insert() {
    let mut jorge = clients::Client::new(3232235826, 8000, "jorge_alarcon", false, 1).unwrap();
    let mac_jorge = ipparser::MacAddress::new_from_str("aaaa.bbbb.cccc").unwrap();
    let gil = clients::Client::new(2352233826, 9000, "gil_vazquez", true, 1).unwrap();
    let mac_gil = ipparser::MacAddress::new_from_str("eeee.1234.fabc").unwrap();
    let tania = clients::Client::new(3232236000, 7000, "tania_m", false, 0).unwrap();
    let mac_tania = ipparser::MacAddress::new_from_str("89ab.9999.ffff").unwrap();
    let fredy = clients::Client::new(3232235788, 9000, "fredy_dela_cruz", false, 0).unwrap();
    let mac_fredy = ipparser::MacAddress::new_from_str("100a.7890.ae45").unwrap();
    let coro = clients::Client::new(3232235799, 9000, "alexis_coro", true, 0).unwrap();
    let mac_coro = ipparser::MacAddress::new_from_str("abcf.2312.9898").unwrap();
    let mut clients_map = clients::ClientsMap::new();

    match clients_map.insert(&mac_jorge, &jorge) {
        clients::InsertionType::Insert => assert!(true),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(false)
    }

    match clients_map.insert(&mac_gil, &gil) {
        clients::InsertionType::Insert => assert!(true),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(false)
    }

    match clients_map.insert(&mac_tania, &tania) {
        clients::InsertionType::Insert => assert!(true),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(false)
    }

    match clients_map.insert(&mac_fredy, &fredy) {
        clients::InsertionType::Insert => assert!(true),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(false)
    }

    match clients_map.insert(&mac_coro, &coro) {
        clients::InsertionType::Insert => assert!(true),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(false)
    }

    // Same mac and ip should result in an update
    match clients_map.insert(&mac_jorge, &jorge) {
        clients::InsertionType::Insert => assert!(false),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(true)
    }

    // Same mac and different ip should result in an update
    jorge.ipv4_addr = 3232235825;
    match clients_map.insert(&mac_jorge, &jorge) {
        clients::InsertionType::Insert => assert!(false),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(false),
        clients::InsertionType::Update => assert!(true)
    }

    // Same ip and different mac should result in a replace
    let mac_jorge = ipparser::MacAddress::new_from_str("aaaa.eeee.cccc").unwrap();
    match clients_map.insert(&mac_jorge, &jorge) {
        clients::InsertionType::Insert => assert!(false),
        clients::InsertionType::Replace { client_mac_replaced: _ } => assert!(true),
        clients::InsertionType::Update => assert!(false)
    }
}

#[test]
fn clients_map_len() {
    let jorge = clients::Client::new(3232235826, 8000, "jorge_alarcon", false, 1).unwrap();
    let mac_jorge = ipparser::MacAddress::new_from_str("aaaa.bbbb.cccc").unwrap();
    let gil = clients::Client::new(2352233826, 9000, "gil_vazquez", true, 1).unwrap();
    let mac_gil = ipparser::MacAddress::new_from_str("eeee.1234.fabc").unwrap();
    let tania = clients::Client::new(3232236000, 7000, "tania_m", false, 0).unwrap();
    let mac_tania = ipparser::MacAddress::new_from_str("89ab.9999.ffff").unwrap();
    let fredy = clients::Client::new(3232235788, 9000, "fredy_dela_cruz", false, 0).unwrap();
    let mac_fredy = ipparser::MacAddress::new_from_str("100a.7890.ae45").unwrap();
    let coro = clients::Client::new(3232235799, 9000, "alexis_coro", true, 0).unwrap();
    let mac_coro = ipparser::MacAddress::new_from_str("abcf.2312.9898").unwrap();
    let mut clients_map = clients::ClientsMap::new();
    clients_map.insert(&mac_jorge, &jorge);
    clients_map.insert(&mac_gil, &gil);
    clients_map.insert(&mac_tania, &tania);
    clients_map.insert(&mac_fredy, &fredy);
    clients_map.insert(&mac_coro, &coro);
    assert_eq!(clients_map.len(), 5);
}

#[test]
fn clients_map_usernames_that_contain() {
    let jorge = clients::Client::new(3232235826, 8000, "jorge_alarcon", false, 1).unwrap();
    let mac_jorge = ipparser::MacAddress::new_from_str("0000.0000.0000").unwrap();
    let gil = clients::Client::new(2352233826, 9000, "gil_vazquez", true, 1).unwrap();
    let mac_gil = ipparser::MacAddress::new_from_str("0000.0000.0001").unwrap();
    let tania = clients::Client::new(3232236000, 7000, "tania_vazquez", false, 0).unwrap();
    let mac_tania = ipparser::MacAddress::new_from_str("0000.0000.0002").unwrap();
    let fredy = clients::Client::new(3232235788, 9000, "fredy_dela_cruz", false, 0).unwrap();
    let mac_fredy = ipparser::MacAddress::new_from_str("0000.0000.0003").unwrap();
    let coro = clients::Client::new(3232235799, 9000, "alexis_coro", true, 0).unwrap();
    let mac_coro = ipparser::MacAddress::new_from_str("0000.0000.0004").unwrap();
    let sabino = clients::Client::new(3212224788, 9000, "sabino_vazquez", true, 0).unwrap();
    let mac_sabino = ipparser::MacAddress::new_from_str("0000.0000.0005").unwrap();
    let martin = clients::Client::new(3221123981, 9000, "martin_vazquez", true, 0).unwrap();
    let mac_martin = ipparser::MacAddress::new_from_str("0000.0000.0006").unwrap();

    let mut clients_map = clients::ClientsMap::new();
    clients_map.insert(&mac_jorge, &jorge);  // 1  0
    clients_map.insert(&mac_gil, &gil);      // 2  1 <-
    clients_map.insert(&mac_tania, &tania);  // 3  2 <-
    clients_map.insert(&mac_fredy, &fredy);  // 4  3
    clients_map.insert(&mac_coro, &coro);    // 5  4
    clients_map.insert(&mac_sabino, &sabino);// 6  5 <-
    clients_map.insert(&mac_martin, &martin);// 7  6 <-

    let (the_vazquez, end_index) = clients_map.usernames_that_contain(0, 2, "vazquez");
    let mut iter = the_vazquez.iter();
    assert_eq!(gil, *iter.next().unwrap());
    assert_eq!(tania, *iter.next().unwrap());
    assert_eq!(end_index, 2);

    let (the_vazquez, end_index) = clients_map.usernames_that_contain(0, 1, "vazquez");
    let mut iter = the_vazquez.iter();
    assert_eq!(gil, *iter.next().unwrap());
    assert_eq!(end_index, 1);

    let (the_vazquez, end_index) = clients_map.usernames_that_contain(0, 3, "vazquez");
    let mut iter = the_vazquez.iter();
    assert_eq!(gil, *iter.next().unwrap());
    assert_eq!(tania, *iter.next().unwrap());
    assert_eq!(sabino, *iter.next().unwrap());
    assert_eq!(end_index, 5);

    let (the_vazquez, end_index) = clients_map.usernames_that_contain(3, 1, "vazquez");
    let mut iter = the_vazquez.iter();
    assert_eq!(sabino, *iter.next().unwrap());
    assert_eq!(end_index, 5);

    let (the_vazquez, end_index) = clients_map.usernames_that_contain(0, 6, "vazquez");
    let mut iter = the_vazquez.iter();
    assert_eq!(gil, *iter.next().unwrap());
    assert_eq!(tania, *iter.next().unwrap());
    assert_eq!(sabino, *iter.next().unwrap());
    assert_eq!(martin, *iter.next().unwrap());
    assert_eq!(end_index, 6);
}
