


from __future__ import annotations
from dataclasses import dataclass

from typing import Any, Optional
import enum



@dataclass
class Node:
    data : Any
    left : Node
    right : Node
    
    

class Side(enum.Enum):
    Left = 1
    Right = 2    


@dataclass
class Skew:
    node : Node
    head : Node = None
    prev : Node = None


    def _make_skew_right(self, node: Node | None ): 
        
        if not node: return
        self._make_skew_right(node.right)
        
        left_node = node.left
        right_node = node.right
        
        
        if not self.head: 
            node.left = None
            self.prev = node
            self.head = node
        else:
            # head already found
            # node.right = prev
            node.left = None
            self.prev.right = node
            self.prev = node
        
        self._make_skew_right(left_node)
    
    def _make_skew_left(self, node: Node | None):
        
        if not node: return
        self._make_skew_right(node.left)
        
        left_node = node.left
        right_node = node.right
        
        
        if not self.head: 
            node.left = None
            self.prev = node
            self.head = node
        else:
            # head already found
            # node.right = prev
            node.left = None
            self.prev.right = node
            self.prev = node
        
        self._make_skew_right(right_node)
    
    def make_skew(self, side: Side):
        if side == Side.Left:
            self._make_skew_left(self.node)
        else:
            self._make_skew_right(self.node)
    
    

# def make_skew(node: Node):
#     if not node: return
    
#     stack = list()
#     stack.append(node)
    
#     while stack:
        
#         curr: Node = stack.pop()
        
#         if curr.left:
            
    
    
def build_tree(iterable) -> Node:
    next_val = next(iterable)
    if next_val is None:
        return None
    
    return Node(next_val , build_tree(iterable), build_tree(iterable))



    

tree = build_tree(
        iter(
            [1,2,4,None, None,5, None,None, 3, 6 ,None,None,None]
        )
        )