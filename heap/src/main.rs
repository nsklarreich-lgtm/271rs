type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    let n = h.len();
    let mut largest = i;

    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && gt(&h[left], &h[largest]) {
        largest = left;
    }

    if right < n && gt(&h[right], &h[largest]) {
        largest = right;
    }

    if largest != i {
        h.swap(i, largest);
        // recursive call returns the modified heap
        return heapify(h, largest, gt);
    }

    // return h as the final expression (no trailing semicolon)
    h
}

fn reheapify<T>(mut h: Heap<T>, mut i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    // bubble up element at i while it is "greater" than its parent
    while i > 0 {
        let parent = (i - 1) / 2;
        if gt(&h[i], &h[parent]) {
            h.swap(i, parent);
            i = parent;
        } else {
            break;
        }
    }

    h
}

fn vec_to_heap<T>(mut xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    let n = xs.len();

    if n > 1 {
        // heapify all parent nodes from the last parent down to 0
        // last_parent = n/2 - 1 (safe because n > 1)
        let last_parent = n / 2 - 1;
        for i in (0..=last_parent).rev() {
            xs = heapify(xs, i, gt);
        }
    }

    xs
}

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    let mut out = Vec::with_capacity(h.len());

    while !h.is_empty() {
        // root (index 0) is the top element; move it out
        out.push(h.swap_remove(0));
        if !h.is_empty() {
            h = heapify(h, 0, gt);
        }
    }

    out
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    heap_to_vec(vec_to_heap(xs, gt), gt)
}

// Example usage
fn gt_i32(a: &i32, b: &i32) -> bool {
    a > b
}

fn main() {
    let xs = vec![3, 1, 4, 1, 5, 9, 2];
    let sorted = hsort(xs, gt_i32);
    println!("{:?}", sorted); // for a max-heap comparator this prints descending order
}

