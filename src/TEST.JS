$(document).ready(function(){
    try{
        const { invoke } = window.__TAURI__.tauri;

        async function save_scores(examId,examType,verbal_score,quant_score, verbalAna_section_2_4) {
        	var verbla_obj = verbalAna_section_2_4;
        	var quant_obj = {};
        	
        	if(examType=="Verbal"){
        	   verbla_obj.examId=examId;
        	   if(verbla_obj.typSce==2){
		   	verbla_obj.tcCorrect= (100*verbla_obj.tcCorrect)/3;
		   	verbla_obj.rcCorrect= (100*verbla_obj.rcCorrect)/5;
		   	verbla_obj.scCorrect= (100*verbla_obj.scCorrect)/4;
		   }else{
		   
		   	verbla_obj.tcCorrect= (100*verbla_obj.tcCorrect)/4;
		   	verbla_obj.rcCorrect= (100*verbla_obj.rcCorrect)/8;
		   	verbla_obj.scCorrect= (100*verbla_obj.scCorrect)/3;

		   }
        	  

		}
		
		
		if(examType=="Verbal"){quant_obj.examId=examId;}
        	
        	
      		var res =  await invoke("save_scores", {examType:examType,verbalScore:verbal_score,quantScore:quant_score, verbalAna:verbla_obj, quantAna:quant_obj});
      		if(res=="200"){
                 $("#pc-app-con").html("Saved!!!")
               }
    	}

    	var verbalAna_section_2_4 = mock_obj.ansedQues.filter(function(e){return e.typSec==2 || e.typSec==4 ;}).reduce(function(acc,ele){
                acc.typSec=ele.typSec;
                if(ele.ans.sort(function(a,b){ return a.blank-b.blank;}).join(',')===ele.selected.sort(function(a,b){ return a.blank-b.blank;}).join(',')){
                    switch (ele.typ) {
                        case 1 : acc.tcCorrect++;
                          break;
                        case 2 : acc.tcCorrect++;
                          break;
                        case 3 : acc.tcCorrect++;
                          break;
                        case 4 : acc.scCorrect++;
                          break;
                        case 5 : acc.rcCorrect++;
                          break;
                        case 6 : acc.rcCorrect++;
                          break;
                        case 63 : acc.rcCorrect++;
                        default:


                    }
                    
                    return acc;
                }else{

		    return acc;
		}
                

    	},{tcCorrect:0,rcCorrect:0,scCorrect:0,typSec:0});

    	$("#pc-app-save").unbind("click").click(function(){
    	    var verbal_score = parseInt($("#gre-score-v").text());
    	    var quant_score = parseInt($("#gre-score-q").text());
            const current_url = window.location.href;
            const parts = current_url.split('/');
            // Get the last part (ID) and the second last part (word "Verbal")
            const examId = parts[parts.length - 1];
            const examType = parts[parts.length - 2];

            save_scores(examId,examType,verbal_score,quant_score,verbalAna_section_2_4);
    	});


    }catch(e){

        console.log(e)
        $("#pc-app-save").css("display","none");
    }

});
