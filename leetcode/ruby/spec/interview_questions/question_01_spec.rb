module LeetcodeInterviewQuestions
  RSpec.describe 'two_sum' do
    it 'finds the indices when they are the first two' do
      expect(
        LeetcodeInterviewQuestions.two_sum([2, 7, 11, 15], 9)
      ).to eq [0, 1]
    end

    it 'finds the indices when they are not adjacent' do
      expect(
        LeetcodeInterviewQuestions.two_sum([3, 2, 4], 6)
      ).to eq [1, 2]
    end

    it 'finds the indices when there are two numbers who sum to the target' do
      expect(
        LeetcodeInterviewQuestions.two_sum([3, 3], 6)
      ).to eq [0, 1]
    end
  end
end
