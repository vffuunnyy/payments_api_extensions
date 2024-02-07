def get_amount(
    claimed_amounts: list[int],
    max_enlargement: int,
    amount: int,
    min_last_amount_difference: int
) -> int | None:
    """
    Returns the amount if it is not claimed, otherwise returns None.
    
    :param claimed_amounts: list of claimed amounts
    :param max_enlargement: maximum amount enlargement
    :param amount: amount to check
    :param min_last_amount_difference: minimum difference between the last amount and the new amount
    
    :return: amount if it is not claimed, otherwise None
    """
