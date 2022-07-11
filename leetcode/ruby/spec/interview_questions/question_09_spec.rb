module LeetcodeInterviewQuestions
  RSpec.describe 'is_palindrome' do
    it 'is true when the number is positive and a palindrome' do
      expect(LeetcodeInterviewQuestions.is_palindrome(121)).to be true
    end

    it 'is false when the number is positive and not a palindrome' do
      expect(LeetcodeInterviewQuestions.is_palindrome(123)).to be false
    end

    it 'is false when the number is negative' do
      expect(LeetcodeInterviewQuestions.is_palindrome(-121)).to be false
    end
  end
end
