class ProductsController < ApplicationController
  before_action :authenticate_user!
  before_action :check_admin, only: [:new, :create]

  def index
    @products = Product.all
  end

  def new
    @product = Product.new
  end

  def create
    @product = Product.new(product_params)
    if @product.save
      redirect_to products_path, notice: 'Товар успешно добавлен'
    else
      render :new
    end
  end
  
  def destroy
    @product = Product.find(params[:id])
    @product.destroy
    redirect_to products_path, notice: 'Товар успешно удален'
  end
  
  private

  def product_params
    params.require(:product).permit(:name, :description, :price, :image)
  end

  def check_admin
    unless current_user.admin?
      redirect_to root_path, alert: 'У вас нет доступа к этой странице'
    end
  end
end

