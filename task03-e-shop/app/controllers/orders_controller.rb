class OrdersController < ApplicationController
  def index
    @user = current_user
    @orders = @user.orders
  end
  
  def new
    @order = Order.new
  end

  def create
    @order = Order.new(order_params)
    if @order.save
      # Успешно создан заказ, теперь нужно добавить логику для обработки заказа

      # Очистка корзины после успешного оформления заказа
      clear_cart

      redirect_to root_path, notice: 'Order placed successfully'
    else
      render :new
    end
  end

  private

  def order_params
    params.require(:order).permit(:user_id, :name, :email, :address, :payment_method)
  end

  def clear_cart
    # Добавьте код для очистки корзины здесь
    # Например, если у вас есть модель Cart, то можете вызвать метод destroy для удаления всех элементов корзины
    current_user.cart.destroy
    # Это предполагает, что у вас есть ассоциация между текущим пользователем и корзиной
  end
end
