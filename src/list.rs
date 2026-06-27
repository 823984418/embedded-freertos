use core::ffi::c_void;
use core::ptr::null_mut;

use crate::*;

pub unsafe fn listSET_LIST_ITEM_OWNER<T>(pxListItem: *mut ListItem_t, pxOwner: *mut T) {
    (*pxListItem).pvOwner = pxOwner as _;
}

pub unsafe fn listGET_LIST_ITEM_OWNER(pxListItem: *mut ListItem_t) -> *mut c_void {
    (*pxListItem).pvOwner
}

pub unsafe fn listSET_LIST_ITEM_VALUE(pxListItem: *mut ListItem_t, xValue: TickType_t) {
    (*pxListItem).xItemValue = xValue;
}

pub unsafe fn listGET_LIST_ITEM_VALUE(pxListItem: *mut ListItem_t) -> TickType_t {
    (*pxListItem).xItemValue
}

pub unsafe fn listGET_ITEM_VALUE_OF_HEAD_ENTRY(pxList: *mut List_t) -> TickType_t {
    (*(*pxList).xListEnd.pxNext).xItemValue
}

pub unsafe fn listGET_HEAD_ENTRY(pxList: *mut List_t) -> *mut ListItem_t {
    (*pxList).xListEnd.pxNext
}

pub unsafe fn listGET_NEXT(pxListItem: *mut ListItem_t) -> *mut ListItem_t {
    (*pxListItem).pxNext
}

pub unsafe fn listGET_END_MARKER(pxList: *mut List_t) -> *const ListItem_t {
    &raw const (*pxList).xListEnd as *const ListItem_t
}

pub unsafe fn listLIST_IS_EMPTY(pxList: *mut List_t) -> BaseType_t {
    if (*pxList).uxNumberOfItems == 0 {
        pdTRUE
    } else {
        pdFALSE
    }
}

pub unsafe fn listCURRENT_LIST_LENGTH(pxList: *mut List_t) -> UBaseType_t {
    (*pxList).uxNumberOfItems
}

#[cfg(configNUMBER_OF_CORES = "1")]
pub unsafe fn listGET_OWNER_OF_NEXT_ENTRY(pxList: *mut List_t) -> TaskHandle_t {
    let pxConstList = pxList;
    (*pxConstList).pxIndex = (*(*pxConstList).pxIndex).pxNext;
    if core::ptr::addr_eq((*pxConstList).pxIndex, &raw mut (*pxConstList).xListEnd) {
        (*pxConstList).pxIndex = (*pxConstList).xListEnd.pxNext;
    }
    (*(*pxConstList).pxIndex).pvOwner as TaskHandle_t
}

pub unsafe fn listREMOVE_ITEM(pxItemToRemove: *mut ListItem_t) {
    let pxList = (*pxItemToRemove).pxContainer;
    (*(*pxItemToRemove).pxNext).pxPrevious = (*pxItemToRemove).pxPrevious;
    (*(*pxItemToRemove).pxPrevious).pxNext = (*pxItemToRemove).pxNext;
    if (*pxList).pxIndex == pxItemToRemove {
        (*pxList).pxIndex = (*pxItemToRemove).pxPrevious;
    }
    (*pxItemToRemove).pxContainer = null_mut();
    (*pxList).uxNumberOfItems = (*pxList).uxNumberOfItems - 1;
}

pub unsafe fn listINSERT_END(pxList: *mut List_t, pxNewListItem: *mut ListItem_t) {
    let pxIndex = (*pxList).pxIndex;

    // FIXME: listTEST_LIST_INTEGRITY(pxList)
    // FIXME: listTEST_LIST_ITEM_INTEGRITY(pxNewListItem)

    (*pxNewListItem).pxNext = pxIndex;
    (*pxNewListItem).pxPrevious = (*pxIndex).pxPrevious;

    (*(*pxIndex).pxPrevious).pxNext = pxNewListItem;
    (*pxIndex).pxPrevious = pxNewListItem;

    (*pxNewListItem).pxContainer = pxList;
    (*pxList).uxNumberOfItems = (*pxList).uxNumberOfItems + 1;
}

pub unsafe fn listGET_OWNER_OF_HEAD_ENTRY(pxList: *mut List_t) -> *mut c_void {
    (*(*pxList).xListEnd.pxNext).pvOwner
}

pub unsafe fn listIS_CONTAINED_WITHIN(
    pxList: *mut List_t,
    pxNewListItem: *mut ListItem_t,
) -> BaseType_t {
    if (*pxNewListItem).pxContainer == pxList {
        pdTRUE
    } else {
        pdFALSE
    }
}

pub unsafe fn listLIST_ITEM_CONTAINER(pxNewListItem: *mut ListItem_t) -> *mut List_t {
    (*pxNewListItem).pxContainer
}

pub unsafe fn listLIST_IS_INITIALISED(pxList: *mut List_t) -> bool {
    (*pxList).xListEnd.xItemValue == TickType_t::MAX
}
