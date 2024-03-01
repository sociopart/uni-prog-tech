class CartItemsController < ApplicationController
  before_action :authenticate_user!
  
  def destroy
    @cart_item = current_user.cart.cart_items.find(params[:id])
    @cart_item.destroy
    redirect_to cart_path, notice: 'Product removed from cart successfully'
  end
end
