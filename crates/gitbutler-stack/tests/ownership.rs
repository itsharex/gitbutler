use std::{path::PathBuf, vec};

use gitbutler_diff::Hunk;
use gitbutler_stack::{reconcile_claims, BranchOwnershipClaims, OwnershipClaim, Stack};

#[test]
fn reconcile_ownership_simple() {
    #[allow(deprecated)] // this is a test
    let mut branch_a = Stack::new(
        "a".to_string(),
        None,
        None,
        None,
        git2::Oid::zero(),
        git2::Oid::zero(),
        usize::default(),
        None,
        true,
    );
    branch_a.ownership = BranchOwnershipClaims {
        claims: vec![OwnershipClaim {
            file_path: PathBuf::from("foo"),
            hunks: vec![
                Hunk {
                    start: 1,
                    end: 3,
                    hash: Some(Hunk::hash("1,3")),
                    hunk_header: None,
                },
                Hunk {
                    start: 4,
                    end: 6,
                    hash: Some(Hunk::hash("4,6")),
                    hunk_header: None,
                },
            ],
        }],
    };
    branch_a.created_timestamp_ms = u128::default();
    branch_a.updated_timestamp_ms = u128::default();

    #[allow(deprecated)] // this is a test
    let mut branch_b = Stack::new(
        "b".to_string(),
        None,
        None,
        None,
        git2::Oid::zero(),
        git2::Oid::zero(),
        usize::default(),
        None,
        true,
    );
    branch_b.ownership = BranchOwnershipClaims {
        claims: vec![OwnershipClaim {
            file_path: PathBuf::from("foo"),
            hunks: vec![Hunk {
                start: 7,
                end: 9,
                hash: Some(Hunk::hash("7,9")),
                hunk_header: None,
            }],
        }],
    };
    branch_b.created_timestamp_ms = u128::default();
    branch_b.updated_timestamp_ms = u128::default();

    let all_branches: Vec<Stack> = vec![branch_a.clone(), branch_b.clone()];
    let claim: Vec<OwnershipClaim> = vec![OwnershipClaim {
        file_path: PathBuf::from("foo"),
        hunks: vec![
            Hunk {
                start: 4,
                end: 6,
                hash: Some(Hunk::hash("4,6")),
                hunk_header: None,
            },
            Hunk {
                start: 7,
                end: 9,
                hash: Some(Hunk::hash("9,7")),
                hunk_header: None,
            },
        ],
    }];
    let claim_outcomes = reconcile_claims(all_branches.clone(), &branch_b, &claim).unwrap();
    assert_eq!(claim_outcomes.len(), all_branches.len());
    assert_eq!(claim_outcomes[0].updated_branch.id, branch_a.id);
    assert_eq!(claim_outcomes[1].updated_branch.id, branch_b.id);

    assert_eq!(
        claim_outcomes[0].updated_branch.ownership,
        BranchOwnershipClaims {
            claims: vec![OwnershipClaim {
                file_path: PathBuf::from("foo"),
                hunks: vec![Hunk {
                    start: 1,
                    end: 3,
                    hash: Some(Hunk::hash("1,3")),
                    hunk_header: None,
                },],
            }],
        }
    );

    assert_eq!(
        claim_outcomes[1].updated_branch.ownership,
        BranchOwnershipClaims {
            claims: vec![OwnershipClaim {
                file_path: PathBuf::from("foo"),
                hunks: vec![
                    Hunk {
                        start: 4,
                        end: 6,
                        hash: Some(Hunk::hash("4,6")),
                        hunk_header: None,
                    },
                    Hunk {
                        start: 7,
                        end: 9,
                        hash: Some(Hunk::hash("9,7")),
                        hunk_header: None,
                    },
                ],
            }],
        }
    );
}

#[test]
fn ownership() {
    let ownership = "src/main.rs:0-100\nsrc/main2.rs:200-300".parse::<BranchOwnershipClaims>();
    assert!(ownership.is_ok());
    let ownership = ownership.unwrap();
    assert_eq!(ownership.claims.len(), 2);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs:0-100".parse::<OwnershipClaim>().unwrap()
    );
    assert_eq!(
        ownership.claims[1],
        "src/main2.rs:200-300".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn ownership_2() {
    let ownership = "src/main.rs:0-100\nsrc/main2.rs:200-300".parse::<BranchOwnershipClaims>();
    assert!(ownership.is_ok());
    let ownership = ownership.unwrap();
    assert_eq!(ownership.claims.len(), 2);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs:0-100".parse::<OwnershipClaim>().unwrap()
    );
    assert_eq!(
        ownership.claims[1],
        "src/main2.rs:200-300".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn put() {
    let mut ownership = "src/main.rs:0-100"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    ownership.put("src/main.rs:200-300".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 1);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs:200-300,0-100"
            .parse::<OwnershipClaim>()
            .unwrap()
    );
}

#[test]
fn put_2() {
    let mut ownership = "src/main.rs:0-100"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    ownership.put("src/main.rs2:200-300".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 2);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs2:200-300".parse::<OwnershipClaim>().unwrap()
    );
    assert_eq!(
        ownership.claims[1],
        "src/main.rs:0-100".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn put_3() {
    let mut ownership = "src/main.rs:0-100\nsrc/main2.rs:100-200"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    ownership.put("src/main2.rs:200-300".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 2);
    assert_eq!(
        ownership.claims[0],
        "src/main2.rs:200-300,100-200"
            .parse::<OwnershipClaim>()
            .unwrap()
    );
    assert_eq!(
        ownership.claims[1],
        "src/main.rs:0-100".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn put_4() {
    let mut ownership = "src/main.rs:0-100\nsrc/main2.rs:100-200"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    ownership.put("src/main2.rs:100-200".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 2);
    assert_eq!(
        ownership.claims[0],
        "src/main2.rs:100-200".parse::<OwnershipClaim>().unwrap()
    );
    assert_eq!(
        ownership.claims[1],
        "src/main.rs:0-100".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn put_7() {
    let mut ownership = "src/main.rs:100-200"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    ownership.put("src/main.rs:100-200".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 1);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs:100-200".parse::<OwnershipClaim>().unwrap()
    );
}

#[test]
fn take_1() {
    let mut ownership = "src/main.rs:100-200,200-300"
        .parse::<BranchOwnershipClaims>()
        .unwrap();
    let taken = ownership.take(&"src/main.rs:100-200".parse::<OwnershipClaim>().unwrap());
    assert_eq!(ownership.claims.len(), 1);
    assert_eq!(
        ownership.claims[0],
        "src/main.rs:200-300".parse::<OwnershipClaim>().unwrap()
    );
    assert_eq!(
        taken,
        vec!["src/main.rs:100-200".parse::<OwnershipClaim>().unwrap()]
    );
}

#[test]
fn equal() {
    for (a, b, expected) in vec![
        (
            "src/main.rs:100-200"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            "src/main.rs:100-200"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            true,
        ),
        (
            "src/main.rs:100-200\nsrc/main1.rs:300-400\n"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            "src/main.rs:100-200"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            false,
        ),
        (
            "src/main.rs:100-200\nsrc/main1.rs:300-400\n"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            "src/main.rs:100-200\nsrc/main1.rs:300-400\n"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            true,
        ),
        (
            "src/main.rs:300-400\nsrc/main1.rs:100-200\n"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            "src/main1.rs:100-200\nsrc/main.rs:300-400\n"
                .parse::<BranchOwnershipClaims>()
                .unwrap(),
            false,
        ),
    ] {
        assert_eq!(a == b, expected, "{:#?} == {:#?}", a, b);
    }
}
