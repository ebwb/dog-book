# lists implementation
> 2.7 - Implement some of the other list operators: copy, merge, split, insert before or after a specific item. How do the two insertion operations differ in difficulty? How much can you use the routines we've written, and how much must you create yourself? (this question assumes one's already written: remove, add, remove all, add-to-front)

## Methods implemented:
|--------|--------------|-------------|

## What all methods will be implemented:
|--------|--------------|-------------|
| add    | merge        | clear       |
| remove | split        | findFirst   |
| copy   | insertBefore | insertAfter |
| get    | sizeOf       |             |


## How should these methods work?
In order to sus out what data structure to use (singly-linked listed,
doubly-linked list, an array-backed list), I'll describe what I would
expect out of an API supporting a list structure.

### add
Add should add at the end of the list.

### merge
When I think of 'merge', I think of what I often hear these days as 'zip'. I'd think that the return of merging two lists, {A, B, C} and {X, Y} should be:

{A, X, B, Y, C}

The calling list would go 'first', as it were.

### clear
I'd expect the modified list to be empty; of size 0. 

### remove
I'd expect to give an index that would be removed.

### get
Get, given a numberic index, should retrieve the item at the index

Given a number that is out of bounds, I would expect some sort of
panic. That may change.

### sizeOf
Parameterless; should return the size of the list.

### split
I'd expect to split at an index and end up with a tuple of references
to two new lists.

### findFirst
Given something that implements 'equals' or is otherwise a primitive, I would expect this to return the value of the first entry of some given argument based on some form of equality; PartialEq or Eq? I'm not sure yet!

### copy
Given a list, I'd expect a new list returned of all the same values; not just the same values, but copies of the values within.

The two then-existing lists shouldn't refer to the same locations in memory. I would expect to be able to modify each list thereafter without worrying about mutations of the values in one list affecting the values of another.

### insertBefore
I'd expect to give an index and that the value argument given is placed before that index such that the inserted value is now at the index given.

### insertAfter
Similar to #insertBefore, I'd expect that the value argument given would be placed at one-after the index given.

## Implementation Plan
As always, start small. Start with i32. Later, generalize.

### The Structure
#### The list, itself
The list ought to know things like:
- the current size
  - if I go with heap-allocated values, allocated on insertion, then I need not worry about something like array-indexing.
  - OTOH, this would make for cheaper validations on several methods, specifically anything that requires specifying an index.
	- then again, are those validations necessary? 
	
#### List items
Cheap adds would be nice; given that I expect that addition of items should happen in LILO fashion, it would be ideal if we didn't have to traverse the entire list to make adds happen (i.e. avoiding O(n)). To that end, a doubly-linked list may suffice; addition starts with moving to the 'previous'-neighbor of the head item.

List items need not be exposed externally; only the values that they track.

### Construction?
Something only implied by the book (from memory, anyway) is construction of the List, itself. That requires some thought. 

How have I seen lists be constructed in the past? Typically, in a Java 8+ world, one may construct a collection using one of any various static factories of any various collections-oriented classes. That seems good and right. For all of those options, I can only recall static methods that take in arguments, themselves. Several of those options would take in several (varargs) options; I'd rather start smaller than that. 

I'll start with 1. It's not the most fluent API, but it's a fine place to start. 
