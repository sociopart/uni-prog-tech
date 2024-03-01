class CartsController < ApplicationController
  before_action :authenticate_user!

  def show
    @cart = current_user.cart || current_user.create_cart
    @cart_items = @cart.cart_items.includes(:product)
  end
  

  def update
    @cart = current_user.cart || current_user.create_cart
    @product = Product.find(params[:product_id])
    @cart_item = @cart.cart_items.find_or_initialize_by(product_id: @product.id)
    @cart_item.quantity ||= 0 # Установить начальное значение количества в нуль, если оно равно nil
    @cart_item.quantity += 1
  
    if @cart_item.save
      redirect_to cart_path, notice: 'Product added to cart successfully'
    else
      redirect_to root_path, alert: 'Failed to add product to cart'
    end
  end
  
end
