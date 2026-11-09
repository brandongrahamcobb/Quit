/* app/src/components/product.rs
 * The purpose of this module is to provide product components.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.rs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct OpenProductsButton;

#[derive(Component)]
pub struct ProductNavigationRoot;

#[derive(Component)]
pub struct ProductListText;

#[derive(Component)]
pub struct ProductAddButton;

#[derive(Component)]
pub struct ProductEditButton(pub i32);

#[derive(Component)]
pub struct ProductFormRoot;

#[derive(Component)]
pub struct ProductSaveButton;

#[derive(Component)]
pub struct ProductCancelButton;

#[derive(Component)]
pub struct ProductErrorText;

#[derive(Component)]
pub struct ProductListContainer;

#[derive(Component)]
pub struct ProductListItem;

#[derive(Component)]
pub struct ProductSearchButton;

#[derive(Component)]
pub struct ProductSearchText;

#[derive(Component)]
pub struct ProductPreviewButton(pub i32);

#[derive(Component)]
pub struct ProductDeleteButton(pub i32);

#[derive(Component)]
pub struct ProductFormOverlay;

#[derive(Component)]
pub struct ProductImageUploadButton;

#[derive(Component)]
pub struct ProductImageRemoveButton;

#[derive(Component)]
pub struct ProductImagePreview;

#[derive(Component)]
pub struct ProductImagePreviewPath(pub String);

#[derive(Component)]
pub struct ProductImagePreviewHandle(pub Handle<Image>);

#[derive(Component)]
pub struct ProductDeleteDialog;

#[derive(Component)]
pub struct ProductDeleteConfirmButton;

#[derive(Component)]
pub struct ProductDeleteCancelButton;

#[derive(Component)]
pub struct ProductDeleteErrorText;

#[derive(Component)]
pub struct ProductDeleteNameText;

#[derive(Component)]
pub struct ProductPreviewImage(pub Handle<Image>);

#[derive(Component)]
pub struct ProductDashboardButton;

#[derive(Component)]
pub struct ProductBrandButton;

#[derive(Component)]
pub struct ProductDescriptionButton;

#[derive(Component)]
pub struct ProductNameButton;

#[derive(Component)]
pub struct ProductRouteButton;

#[derive(Component)]
pub struct ProductRouteOptionButton(pub String);

#[derive(Component)]
pub struct ProductCompoundToggleButton(pub i32);

#[derive(Component)]
pub struct ProductNameText;

#[derive(Component)]
pub struct ProductRouteText;

#[derive(Component)]
pub struct ProductBrandText;

#[derive(Component)]
pub struct ProductDescriptionText;

#[derive(Component)]
pub struct ProductCompoundChoiceText;

#[derive(Component)]
pub struct ProductCompoundListContainer;

#[derive(Component)]
pub struct ProductCompositionItem;

#[derive(Component)]
pub struct ProductCompositionAmountButton(pub i32);

#[derive(Component)]
pub struct ProductCompositionAmountText(pub i32);

#[derive(Component)]
pub struct ProductCompositionUnitButton(pub i32);

#[derive(Component)]
pub struct ProductCompositionRemoveButton(pub i32);

#[derive(Component)]
pub struct ProductRouteOptionList;

#[derive(Component)]
pub struct ProductCompositionListContainer;

#[derive(Component)]
pub struct ProductSaveButtonText;

#[derive(Component)]
pub struct ProductServingSizeButton;

#[derive(Component)]
pub struct ProductServingSizeText;

#[derive(Component)]
pub struct ProductServingUnitButton;

#[derive(Component)]
pub struct ProductServingUnitText;

#[derive(Component)]
pub struct ProductServingDescriptionButton;

#[derive(Component)]
pub struct ProductServingDescriptionText;

#[derive(Component)]
pub struct ProductServingUnitOptionList;

#[derive(Component)]
pub struct ProductServingUnitOptionButton(pub String);

#[derive(Component)]
pub struct ProductPreviewOverlay;

#[derive(Component)]
pub struct ProductPreviewPanel;

#[derive(Component)]
pub struct ProductPreviewContent;

#[derive(Component)]
pub struct ProductCompositionSearchText;
