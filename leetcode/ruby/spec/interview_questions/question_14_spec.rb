module LeetcodeInterviewQuestions
  RSpec.describe 'longest_common_prefix' do
    it 'finds the longest prefix when there is a common prefix' do
      expect(
        LeetcodeInterviewQuestions.longest_common_prefix(%w[flower flow flight])
      ).to eq 'fl'
    end

    it 'does not find a prefix when there is not a common prefix' do
      expect(
        LeetcodeInterviewQuestions.longest_common_prefix(%w[dog racecar car])
      ).to eq ''
    end

    it 'finds a common prefix when only the first letter is common' do
      expect(
        LeetcodeInterviewQuestions.longest_common_prefix(%w[cir car])
      ).to eq 'c'
    end
  end
end
