module LeetcodeInterviewQuestions
  RSpec.describe 'roman_to_int' do
    it 'converts III to 3' do
      expect(LeetcodeInterviewQuestions.roman_to_int('III')).to eq 3
    end

    it 'converts LVIII to 58' do
      expect(LeetcodeInterviewQuestions.roman_to_int('LVIII')).to eq 58
    end

    it 'MCMXCIV to 1994' do
      expect(LeetcodeInterviewQuestions.roman_to_int('MCMXCIV')).to eq 1994
    end
  end
end
