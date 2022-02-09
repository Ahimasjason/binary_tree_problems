from __future__ import annotations

from typing import Any
from dataclasses import dataclass


@dataclass
class Node():
    data: Any
    left : Node
    right : Node
    lthred : bool 
    rthread : bool 
    
    
    
    
    
    
def build(iterable):
    val = next(iterable)
    print(val)
    if val is None:
        return None
    return Node(data=val,left=build(iterable), right=build(iterable), lthred=False,rthread=False)



tree = build(
    iter(
        [15,5,1,None,None,7,None,None,15,12,None,None,20,None,None]
    )
)
    

@dataclass
class Tree():
    root : Node = None
    def del_case1(self, key : Any):
        ptr = self.root
        par = None
        while ptr != None:
            if ptr.data == key:
                break
            par = ptr
            if key < ptr.data:
                ptr = ptr.left
            else:
                ptr = ptr.right
                
    def search(self, key : Any) -> Node:
        rv = self.root        
        while rv is not None:
            if rv.data == key:
                return rv
            elif key < rv.data:
                rv = rv.left
            else:
                rv.right


    def insert(self, data : Any):
        ptr = self.root
        par = None
        while ptr is not None:
            if ptr.data == data:
                raise ValueError("Value already exist")
            par = ptr
            if data < ptr.data:
                if ptr.lthred is False:
                    ptr = ptr.left
                else:
                    # reached leaf node
                    break;
            
            else:
                if ptr.rthread is False:
                    ptr = ptr.right
                else:
                    break
                
        
        new = Node(data,None,None,True,True)
        if par is None:
            self.root = new
                            
        elif new.data < par.data:
            #
            new.left = par.left
            new.right = par
            par.lthred = False
            par.left = new
        else:
            new.right = par.right
            new.left = par
            par.rthread = False
            par.right = new
            
        return
    

    
    
    
    
    
    
    
class Convert():
    
    def __init__(self, root) -> None:
        self.root = root
    
    def succ_convert(self, root: Node, prev = None):
        if root is None:
            return
        print("Before --> ", root.data, prev.data if prev else None)
        self.succ_convert(root.right, prev)
        if root.right is None and prev is not None:
            print(root.data, prev.data)
            root.right = prev
            root.rthread = True
        self.succ_convert(root.left, root)
        
    def pred_convert(self, root: Node, prev: Node = None):
        if not root:return
        self.pred_convert(root.left, prev)
        if root.left is None and prev is not None:
            root.left = prev
            root.lthred = True
        if not root.rthread:
            self.pred_convert(root.right, root)
    
    
    def convert(self):
        self.succ_convert(self.root)
        self.pred_convert(self.root)
            
    

    
    