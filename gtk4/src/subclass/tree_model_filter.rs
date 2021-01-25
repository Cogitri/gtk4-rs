// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object, Value};

use crate::{TreeIter, TreeModel, TreeModelFilter};

pub trait TreeModelFilterImpl: TreeModelFilterImplExt + ObjectImpl {
    fn visible(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
    ) -> bool {
        self.parent_visible(tree_model_filter, child_model, iter)
    }
    fn modify(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
        value: Value,
        column: i32,
    ) {
        self.parent_modify(tree_model_filter, child_model, iter, value, column)
    }
}

pub trait TreeModelFilterImplExt: ObjectSubclass {
    fn parent_visible(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
    ) -> bool;
    fn parent_modify(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
        value: Value,
        index: i32,
    );
}

impl<T: TreeModelFilterImpl> TreeModelFilterImplExt for T {
    fn parent_visible(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkTreeModelFilterClass;
            if let Some(f) = (*parent_class).visible {
                from_glib(f(
                    tree_model_filter
                        .unsafe_cast_ref::<TreeModelFilter>()
                        .to_glib_none()
                        .0,
                    child_model.to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                ))
            } else {
                true // always visible if not set
            }
        }
    }

    fn parent_modify(
        &self,
        tree_model_filter: &Self::Type,
        child_model: &TreeModel,
        iter: &TreeIter,
        value: Value,
        index: i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkTreeModelFilterClass;
            if let Some(f) = (*parent_class).modify {
                f(
                    tree_model_filter
                        .unsafe_cast_ref::<TreeModelFilter>()
                        .to_glib_none()
                        .0,
                    child_model.to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    value.to_glib_none().0 as *mut _,
                    index,
                )
            }
        }
    }
}

unsafe impl<T: TreeModelFilterImpl> IsSubclassable<T> for TreeModelFilter {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.visible = Some(tree_model_filter_visible::<T>);
        klass.modify = Some(tree_model_filter_modify::<T>);
    }
}

unsafe extern "C" fn tree_model_filter_visible<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<TreeModelFilter> = from_glib_borrow(ptr);
    let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.visible(wrap.unsafe_cast_ref(), &child_model, &iter)
        .to_glib()
}

unsafe extern "C" fn tree_model_filter_modify<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    valueptr: *mut glib::gobject_ffi::GValue,
    column: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<TreeModelFilter> = from_glib_borrow(ptr);
    let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);
    let value: Value = from_glib_full(valueptr);

    imp.modify(wrap.unsafe_cast_ref(), &child_model, &iter, value, column)
}
