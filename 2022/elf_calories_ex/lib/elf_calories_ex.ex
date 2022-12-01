defmodule ElfCaloriesEx do
  # puzzle 1
  def top_carrier do
    File.stream!("./lib/input.txt", [encoding: :utf8], :line)
    |> Stream.map(fn
      "\n" -> ""
      line when is_binary(line) -> String.trim(line) |> String.to_integer()
    end)
    |> Enum.reduce(%{most: 0, current: 0}, fn
      "", %{most: most, current: current} when current > most -> %{most: current, current: 0}
      "", %{most: most, current: current} = acc when current <= most -> Map.put(acc, :current, 0)
      calories, %{current: current} = acc -> Map.put(acc, :current, current + calories)
    end)
    |> Map.get(:most)
  end

  # puzzle 2
  def top_three_carriers do
    File.stream!("./lib/input.txt", [encoding: :utf8], :line)
    |> Stream.map(fn
      "\n" -> ""
      line when is_binary(line) -> String.trim(line) |> String.to_integer()
    end)
    |> Enum.reduce(%{most: [0, 0, 0], current: 0}, fn
      # Current elf's total is more than the top elf. This elf takes the top spot
      # and bumps down the top and second elf
      "", %{most: [top, second | _rest], current: current} when current > top ->
        %{most: [current, top, second], current: 0}

      # Current elf's total is more than the second place elf, so it bumps
      # down the second place elf
      "", %{most: [top, second | _rest], current: current} when current > second ->
        %{most: [top, current, second], current: 0}

      # Current elf's total is more than third place elf, so we replace the third place
      # elf
      "", %{most: [top, second, third], current: current} when current > third ->
        %{most: [top, second, current], current: 0}

      # This elf didn't place, reset for the next elf
      "", acc ->
        Map.put(acc, :current, 0)

      # We're counting for one elf
      calories, %{current: current} = acc ->
        Map.put(acc, :current, current + calories)
    end)
    |> Map.get(:most)
    |> Enum.sum()
  end
  
  # less "all in one function" refactor of puzzle 2 solution
  def top_three_carriers_funcs do
    load_sample()
    |> Enum.reduce(%{most: [0, 0, 0], current: 0}, &calorie_placement/2)
    |> Map.get(:most)
    |> Enum.sum()
  end
  
  defp load_sample() do
    File.stream!("./lib/input.txt", [encoding: :utf8], :line)
    |> Stream.map(fn
      "\n" -> ""
      line when is_binary(line) -> String.trim(line) |> String.to_integer()
    end)
  end
  
  # Current elf's total is more than the top elf. This elf takes the top spot
  # and bumps down the top and second elf
  defp calorie_placement("", %{most: [top, second | _rest], current: current}) when current > top do
    %{most: [current, top, second], current: 0}
  end
  
  # Current elf's total is more than the second place elf, so it bumps
  # down the second place elf
  defp calorie_placement("", %{most: [top, second | _rest], current: current}) when current > second do
    %{most: [top, current, second], current: 0}
  end
  
  # Current elf's total is more than third place elf, so we replace the third place
  # elf
  defp calorie_placement("", %{most: [top, second, third], current: current}) when current > third do
    %{most: [top, second, current], current: 0}
  end
  
  # This elf didn't place, reset for the next elf
  defp calorie_placement("", acc) do
    Map.put(acc, :current, 0)
  end
  
  # We're counting for one elf
  defp calorie_placement(calories, %{current: current} = acc) do
    Map.put(acc, :current, current + calories)
  end
end
