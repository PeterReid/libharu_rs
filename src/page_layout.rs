use libharu_sys as haru;

/// Describes how a page should be displayed. If this attribute is not set, the setting of a viewer application is used.
pub enum PageLayout {
    /// Only one page is displayed.
    Single,
    
    /// Display the pages in one column.
    OneColumn,
    
    /// Display the pages in two column. The page of the odd number is displayed left. 
    TwoColumnLeft,
    
    /// Display the pages in two column. The page of the odd number is displayed right. 
    TwoColumnRight,
}

impl PageLayout {
    pub fn as_int(&self) -> haru::HPDF_PageLayout {
        match *self {
            PageLayout::Single => haru::HPDF_PAGE_LAYOUT_SINGLE,
            PageLayout::OneColumn => haru::HPDF_PAGE_LAYOUT_ONE_COLUMN,
            PageLayout::TwoColumnLeft => haru::HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
            PageLayout::TwoColumnRight => haru::HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
        }
    }
    
    pub fn from_int(layout: haru::HPDF_PageLayout) -> Option<PageLayout> {
        Some(match layout {
            haru::HPDF_PAGE_LAYOUT_SINGLE => PageLayout::Single,
            haru::HPDF_PAGE_LAYOUT_ONE_COLUMN => PageLayout::OneColumn,
            haru::HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => PageLayout::TwoColumnLeft,
            haru::HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT => PageLayout::TwoColumnRight,
            _ => { return None; } // probably HPDF_PAGE_LAYOUT_EOF, meaning unset
        })
    }
}
