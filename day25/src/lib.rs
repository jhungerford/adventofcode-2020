/// Performs a handshake between the card and the door, and returns the resulting encryption key.
pub fn handshake(card_pk: i64, door_pk: i64) -> i64 {
    let card_loop_size = find_loop(7, card_pk);
    let door_loop_size = find_loop(7, door_pk);

    let card_encryption_key = transform(door_pk, card_loop_size);
    let door_encryption_key = transform(card_pk, door_loop_size);

    assert_eq!(card_encryption_key, door_encryption_key,
               "Card and door encryption keys must match.  \
               CARD pk:{}, loop:{}, encryption:{} DOOR pk:{}, loop:{}, encryption:{}",
               card_pk, card_loop_size, card_encryption_key, door_pk, door_loop_size, door_encryption_key);

    card_encryption_key
}

/// Transforms the given subject number loop_size times.  A single transform sets the value
/// to itself multiplied by the subject number, then takes the remainder after dividing by 20201227.
fn transform(subject_num: i64, loop_size: i64) -> i64 {
    let mut value = 1;
    for _ in 0 .. loop_size {
        value = (value * subject_num) % 20201227;
    }

    value
}

/// Finds the loop value that lets the subject number be transformed into the target value.
fn find_loop(subject_num: i64, target: i64) -> i64 {
    let mut value = 1;
    let mut loop_size = 0;

    while value != target {
        value = (value * subject_num) % 20201227;
        loop_size += 1;
    }

    loop_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_loop_sample() {
        assert_eq!(8, find_loop(7, 5764801));
        assert_eq!(11, find_loop(7, 17807724));
    }

    #[test]
    fn transform_sample() {
        assert_eq!(5764801, transform(7, 8));
        assert_eq!(17807724, transform(7, 11));

        assert_eq!(14897079, transform(17807724, 8));
        assert_eq!(14897079, transform(5764801, 11));
    }

    #[test]
    fn handshake_sample() {
        assert_eq!(14897079, handshake(5764801, 17807724));
    }
}
