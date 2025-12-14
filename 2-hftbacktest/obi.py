from numba import njit
import numpy as np

@njit
def calculate_obi(bid_qty, ask_qty):
    """
    Calculates the Order Book Imbalance (OBI) from bid and ask quantities.

    Args:
        bid_qty (np.ndarray): Array of quantities for bids.
        ask_qty (np.ndarray): Array of quantities for asks.

    Returns:
        float: The calculated Order Book Imbalance.
    """
    total_bid_qty = np.sum(bid_qty)
    total_ask_qty = np.sum(ask_qty)
    
    if total_bid_qty + total_ask_qty == 0:
        return 0.5  # Return a neutral value if there are no quantities
        
    return total_bid_qty / (total_bid_qty + total_ask_qty)

@njit
def weighted_mid_price(best_bid, best_ask, obi):
    """
    Calculates the weighted mid-price based on the Order Book Imbalance.

    Args:
        best_bid (float): The best bid price.
        best_ask (float): The best ask price.
        obi (float): The Order Book Imbalance.

    Returns:
        float: The weighted mid-price.
    """
    return best_ask * obi + best_bid * (1 - obi)
