Rails.application.routes.draw do
  devise_for :users
  resources :products do
    delete 'destroy', on: :member
  end
  resource :cart, only: [:show, :update]
  root 'products#index'
end
