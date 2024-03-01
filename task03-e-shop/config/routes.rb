Rails.application.routes.draw do
  get 'orders/new'
  post 'orders/create'
  devise_for :users
  resources :products do
    delete 'destroy', on: :member
  end
  resources :orders
  resource :cart, only: [:show, :update]
  resources :cart_items, only: [:destroy]
  post '/cart', to: 'carts#update'
  root 'products#index'
end
