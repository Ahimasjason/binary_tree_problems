
from __future__ import annotations
from dataclasses import dataclass

from typing import Any, Optional

@dataclass
class Node:
    data : Any
    left : Node
    right : Node
    
    
    
    
def build_tree(iterable) -> Node:
    next_val = next(iterable)
    if next_val is None:
        return None
    
    return Node(next_val , build_tree(iterable), build_tree(iterable))



@dataclass
class DLL:
    node: Node
    prev_node : None
    
    def prev(self, node):
        
        if not node:
            return
        self.prev(node.left)
        node.left = self.prev_node
        self.prev_node = node
        self.prev(node.right)
        
        
    def _next(self, root: Node):
        
        prev_node = None
        
        while root and root.right :
            # go to last node and 
            root = root.right
        


        while root and root.left:
            prev_node = root
            root = root.left
            root.right = prev_node
            
            
        return root
    
    def convert(self):
        self.prev(self.node)
        return self._next(self.node)




@dataclass
class DLL2:
    node : Node
    head : Node = None
    prev: Node = None
    
    def _convert(self, root: Node) -> None:
        
        if not root: return
        
        self._convert(root.left)
        # Inorder traversal 
        if not self.prev:
            self.head = root
        else:
            # 
            root.left = self.prev
            self.prev.right = root
        self.prev = root
        self._convert(root.right)
            
    def convert(self) -> Node:
        n = self.node
        self._convert(n)
        return n


tree = build_tree(
        iter(
            [1,2,4,None, None,5, None,None, 3, 6 ,None,None,None]
        )
        )






    
    
    