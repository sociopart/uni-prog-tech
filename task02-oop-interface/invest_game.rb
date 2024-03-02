class InvestmentFund
  attr_accessor :capital, :portfolio, :monthly_profit, :tax_rate

  def initialize(capital, portfolio, tax_rate)
    @capital = capital
    @portfolio = portfolio
    @tax_rate = tax_rate
  end

  def calculate_profit
    @monthly_profit = 0
    @portfolio.each do |investment|
      # Рассчитываем прибыль для каждой составляющей портфеля
      @monthly_profit += investment.calculate_monthly_profit
    end
    @monthly_profit
  end

  def pay_tax
    tax_amount = @monthly_profit * @tax_rate
    @capital -= tax_amount
    tax_amount
  end

  def adjust_portfolio(target_allocation)
    total_value = @portfolio.sum(&:value)
    target_allocation.each do |investment_name, target_percentage|
      investment = @portfolio.find { |inv| inv.name == investment_name }
      next unless investment

      target_value = total_value * target_percentage
      delta = target_value - investment.value
      next if delta.abs < 0.01 # Ничего не делаем, если разница мала

      if delta.positive? # Покупаем активы
        # увеличиваем стоимость активов на случайную величину
        investment.value += rand(0.1..0.3) * delta
      else # Продаем активы
        # уменьшаем стоимость активов на случайную величину
        investment.value -= rand(0.1..0.3) * delta.abs
      end
    end
  end

  def monthly_report
    puts "Суммарный капитал: #{@capital}"
    puts "Месячная прибыль: #{@monthly_profit}"
    puts "Состав портфеля:"
    @portfolio.each do |investment|
      puts "#{investment.name}: стоимость #{investment.value}"
    end
  end
end

class Investment
  attr_accessor :name, :value, :risk, :mean_return, :std_dev

  def initialize(name, value, risk, mean_return, std_dev)
    @name = name
    @value = value
    @risk = risk
    @mean_return = mean_return
    @std_dev = std_dev
  end

  def calculate_monthly_profit
    # Здесь рассчитываем прибыль на основе случайных изменений
    # с учетом средней доходности и стандартного отклонения
    monthly_return = rand_normal(@mean_return, @std_dev)
    monthly_profit = @value * monthly_return
    monthly_profit
  end

  def adjust_value
    # Здесь реализуем логику изменения стоимости инвестиции
    # В данном случае мы просто моделируем случайные изменения стоимости
    change_percentage = rand_normal(@mean_return, @std_dev)
    @value *= (1 + change_percentage)
  end

  private def rand_normal(mean, std_dev)
    # Генерируем случайное число из нормального распределения
    # с заданным средним и стандартным отклонением
    normal_random = rand * 2 - 1 + rand * 2 - 1 + rand * 2 - 1
    normal_random = (normal_random / 3.0) * std_dev + mean
    normal_random
  end
end

class FundManager
  def initialize(fund)
    @fund = fund
  end

  def play_month
    @fund.calculate_profit
    tax_paid = @fund.pay_tax
    @fund.adjust_portfolio(@fund.portfolio)
    tax_paid
  end

  def view_report
    @fund.monthly_report
  end
end


class InvestmentGameCLI
  def initialize
    @initial_capital = 560000 # начальный капитал
    @initial_portfolio = [
      Investment.new("Депозит", 10000, 0.1, 0.05, 0.02),
      Investment.new("Акции", 20000, 0.2, 0.1, 0.05),
      # Другие виды инвестиций
    ]
    @tax_rate = 0.17 # процент налога на доход
    @months = 12
    @target_allocation = {
      "Депозит" => 0.3,
      "Акции" => 0.6,
      # Другие виды инвестиций
    }
  end

  def start_game
    fund = InvestmentFund.new(@initial_capital, @initial_portfolio, @tax_rate)
    manager = FundManager.new(fund)

    @months.times do |month|
      puts "Месяц #{month + 1}:"
      manager.play_month
      fund.adjust_portfolio(@target_allocation)
      manager.view_report
      puts "Нажмите Enter для продолжения..."
      gets.chomp
    end
  end
end

game = InvestmentGameCLI.new
game.start_game