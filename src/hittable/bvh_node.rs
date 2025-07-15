use super::{aabb::Aabb, HitRecord, Hittable};
use crate::ray::Ray;
use std::{ops::Range, sync::Arc};

#[derive(Clone)]
struct NodeData {
    aabb: Aabb,
    left_child: Option<usize>,
    right_child: Option<usize>,
    object: Option<Arc<dyn Hittable>>,
}

#[derive(Clone)]
pub enum BvhNode {
    Leaf {
        aabb: Aabb,
        object: Arc<dyn Hittable>,
    },
    Internal {
        aabb: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        if objects.is_empty() {
            panic!("Cannot create BVH from empty object list");
        }
        Self::build_spatial_median_iterative(objects)
    }

    fn build_spatial_median_iterative(objects: Vec<Arc<dyn Hittable>>) -> Self {
        if objects.len() == 1 {
            let object = objects.into_iter().next().unwrap();
            return BvhNode::Leaf {
                aabb: object.aabb(),
                object,
            };
        }

        struct WorkItem {
            objects: Vec<Arc<dyn Hittable>>,
            node_index: usize,
        }

        let mut nodes: Vec<NodeData> = Vec::new();
        let mut work_stack: Vec<WorkItem> = Vec::new();

        let root_index = 0;
        nodes.push(NodeData {
            aabb: Aabb::default(),
            left_child: None,
            right_child: None,
            object: None,
        });

        work_stack.push(WorkItem {
            objects,
            node_index: root_index,
        });

        while let Some(work_item) = work_stack.pop() {
            let current_objects = work_item.objects;
            let node_index = work_item.node_index;

            if current_objects.len() == 1 {
                let object = current_objects.into_iter().next().unwrap();
                if node_index < nodes.len() {
                    nodes[node_index] = NodeData {
                        aabb: object.aabb(),
                        left_child: None,
                        right_child: None,
                        object: Some(object),
                    };
                }
            } else {
                let (left_objects, right_objects) = Self::spatial_median_split(current_objects);

                let left_index = nodes.len();
                nodes.push(NodeData {
                    aabb: Aabb::default(),
                    left_child: None,
                    right_child: None,
                    object: None,
                });

                let right_index = nodes.len();
                nodes.push(NodeData {
                    aabb: Aabb::default(),
                    left_child: None,
                    right_child: None,
                    object: None,
                });

                if node_index < nodes.len() {
                    nodes[node_index].left_child = Some(left_index);
                    nodes[node_index].right_child = Some(right_index);
                }

                work_stack.push(WorkItem {
                    objects: right_objects,
                    node_index: right_index,
                });
                work_stack.push(WorkItem {
                    objects: left_objects,
                    node_index: left_index,
                });
            }
        }

        Self::convert_to_bvh_node(&nodes, root_index)
    }

    fn spatial_median_split(
        objects: Vec<Arc<dyn Hittable>>,
    ) -> (Vec<Arc<dyn Hittable>>, Vec<Arc<dyn Hittable>>) {
        let mut overall_aabb = objects[0].aabb();
        for obj in &objects[1..] {
            overall_aabb = Aabb::from((overall_aabb, obj.aabb()));
        }

        let axis = Self::longest_axis(&overall_aabb);
        let split_pos = overall_aabb.center[axis]; // Spatial median

        let mut left_objects = Vec::new();
        let mut right_objects = Vec::new();

        for obj in objects {
            if obj.aabb().center[axis] < split_pos {
                left_objects.push(obj);
            } else {
                right_objects.push(obj);
            }
        }

        if left_objects.is_empty() || right_objects.is_empty() {
            let mut all_objects = left_objects;
            all_objects.extend(right_objects);

            all_objects.sort_by(|a, b| {
                let a_center = a.aabb().center[axis];
                let b_center = b.aabb().center[axis];
                a_center
                    .partial_cmp(&b_center)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let mid = all_objects.len() / 2;
            right_objects = all_objects.split_off(mid);
            left_objects = all_objects;
        }

        (left_objects, right_objects)
    }

    #[inline]
    fn longest_axis(aabb: &Aabb) -> usize {
        let extents = aabb.half_extents * 2.0;
        if extents.x >= extents.y && extents.x >= extents.z {
            0 // X axis
        } else if extents.y >= extents.z {
            1 // Y axis
        } else {
            2 // Z axis
        }
    }

    fn convert_to_bvh_node(nodes: &[NodeData], index: usize) -> Self {
        if index >= nodes.len() {
            panic!("Invalid node index during BVH construction");
        }

        let node = &nodes[index];

        match (&node.left_child, &node.right_child, &node.object) {
            (None, None, Some(object)) => BvhNode::Leaf {
                aabb: node.aabb,
                object: object.clone(),
            },
            (Some(left_idx), Some(right_idx), None) => {
                let left_child = Box::new(Self::convert_to_bvh_node(nodes, *left_idx));
                let right_child = Box::new(Self::convert_to_bvh_node(nodes, *right_idx));

                let combined_aabb = Aabb::from((left_child.aabb(), right_child.aabb()));

                BvhNode::Internal {
                    aabb: combined_aabb,
                    left: left_child,
                    right: right_child,
                }
            }
            _ => panic!("Invalid node structure during BVH construction"),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        match self {
            BvhNode::Leaf { aabb, object } => {
                if aabb.hit(ray, ray_t.clone()).is_some() {
                    object.hit(ray, ray_t)
                } else {
                    None
                }
            }
            BvhNode::Internal { aabb, left, right } => {
                if aabb.hit(ray, ray_t.clone()).is_none() {
                    return None;
                }

                let left_hit = left.hit(ray, ray_t.clone());
                let right_hit = right.hit(ray, ray_t.clone());

                match (left_hit, right_hit) {
                    (Some(left_record), Some(right_record)) => {
                        if left_record.t < right_record.t {
                            Some(left_record)
                        } else {
                            Some(right_record)
                        }
                    }
                    (Some(record), None) | (None, Some(record)) => Some(record),
                    (None, None) => None,
                }
            }
        }
    }

    #[inline]
    fn aabb(&self) -> Aabb {
        match self {
            BvhNode::Leaf { aabb, .. } => *aabb,
            BvhNode::Internal { aabb, .. } => *aabb,
        }
    }
}
